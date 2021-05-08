#![no_std]
use core::f32::consts::PI;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::pixelcolor::RgbColor;
use embedded_graphics::prelude::Drawable;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::Circle;
use embedded_graphics::primitives::Line;
use embedded_graphics::style::PrimitiveStyle;
use embedded_graphics::DrawTarget;
use micromath::F32Ext;
use rand::prelude::*;
const COHESION_FORCE: f32 = 0.004;
const SEPARATION_FORCE: f32 = 0.5;
const ALIGNMENT_FORCE: f32 = 0.05;
const BOUNDARY_FORCE: f32 = 0.01;
const COHESION_DISTANCE: f32 = 0.4;
const SEPARATION_DISTANCE: f32 = 0.05;
const ALIGNMENT_DISTANCE: f32 = 0.1;
const COHESION_ANGLE: f32 = PI / 2.0;
const SEPARATION_ANGLE: f32 = PI / 2.0;
const ALIGNMENT_ANGLE: f32 = PI / 3.0;
const MIN_VELOCITY: f32 = 0.005;
const MAX_VELOCITY: f32 = 0.03;
const N: usize = 2;
const M: usize = 50;
const SCALE: f32 = 100.0;
const CENTER_X: i32 = 160;
const CENTER_Y: i32 = 120;
const RAD: u32 = 3;
#[derive(Debug, Clone, Copy)]
pub struct Boid {
    position: [f32; N],
    _prev_position: [f32; N],
    velocity: [f32; N],
}

fn norm(x: [f32; N]) -> f32 {
    dot(x, x).sqrt()
}

fn calc_distance(source: &Boid, target: &Boid) -> f32 {
    norm(minus(target.position, source.position))
}

fn calc_coherence(source: &Boid, boids: &[Boid; M], dist: [f32; M], angle: [f32; M]) -> [f32; N] {
    let mut coh = [0.0; N];
    let mut cnt = 0;
    for i in 0..N {
        if dist[i] < COHESION_DISTANCE && angle[i] < COHESION_ANGLE {
            cnt += 1;
            coh = plus(coh, boids[i].position);
        }
    }
    if cnt > 0 {
        let average = divide(coh, cnt as f32);
        multiply(minus(average, source.velocity), COHESION_FORCE)
    } else {
        coh // [0.0; N]
    }
}

fn calc_separation(source: &Boid, boids: &[Boid; M], dist: [f32; M], angle: [f32; M]) -> [f32; N] {
    let mut sep = [0.0; N];
    let mut cnt = 0;
    for i in 0..N {
        if dist[i] < SEPARATION_DISTANCE && angle[i] < SEPARATION_ANGLE {
            cnt += 1;
            sep = plus(sep, minus(source.position, boids[i].position));
        }
    }
    if cnt > 0 {
        multiply(sep, SEPARATION_FORCE)
    } else {
        sep
    }
}

fn calc_alignment(source: &Boid, boids: &[Boid; M], dist: [f32; M], angle: [f32; M]) -> [f32; N] {
    let mut ali = [0.0; N];
    let mut cnt = 0;
    for i in 0..N {
        if dist[i] < ALIGNMENT_DISTANCE && angle[i] < ALIGNMENT_ANGLE {
            cnt += 1;
            ali = plus(ali, boids[i].velocity);
        }
    }
    if cnt > 0 {
        let average = divide(ali, cnt as f32);
        multiply(minus(average, source.velocity), ALIGNMENT_FORCE)
    } else {
        ali
    }
}

fn calc_boundary(source: &Boid, _: &[Boid; M], _: [f32; M], _: [f32; M]) -> [f32; N] {
    let dist_center = norm(source.position);
    if dist_center > 1.0 {
        multiply(
            source.position,
            (dist_center - 1.0) * -BOUNDARY_FORCE / dist_center,
        )
    } else {
        [0.0; N]
    }
}
fn dot(x: [f32; N], y: [f32; N]) -> f32 {
    let mut sum = 0.0;
    for i in 0..N {
        sum += x[i] * y[i];
    }
    sum
}

fn minus(x: [f32; N], y: [f32; N]) -> [f32; N] {
    let mut arr = [0.0; N];
    for i in 0..N {
        arr[i] = x[i] - y[i];
    }
    arr
}

fn plus(x: [f32; N], y: [f32; N]) -> [f32; N] {
    let mut arr = [0.0; N];
    for i in 0..N {
        arr[i] = x[i] + y[i];
    }
    arr
}

fn divide(x: [f32; N], d: f32) -> [f32; N] {
    let mut arr = [0.0; N];
    for i in 0..N {
        arr[i] = x[i] / d;
    }
    arr
}

fn multiply(x: [f32; N], d: f32) -> [f32; N] {
    let mut arr = [0.0; N];
    for i in 0..N {
        arr[i] = x[i] * d;
    }
    arr
}

fn calc_angle(source: &Boid, target: &Boid) -> f32 {
    let d = minus(target.position, source.position);
    let theta: f32 = dot(source.velocity, d) / (norm(source.velocity) * norm(d));
    let angle = theta.acos();
    angle
}

impl Boid {
    fn new() -> Self {
        Boid {
            position: [0.0; N],
            _prev_position: [0.0; N],
            velocity: [0.0; N],
        }
    }
}

impl Default for Boid {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Boids {
    boids: [Boid; M],
    _dv_coh: [[f32; N]; M],
    _dv_sep: [[f32; N]; M],
    _dv_ali: [[f32; N]; M],
    _dv_bnd: [[f32; N]; M],
}

impl Boids {
    pub fn new() -> Self {
        Boids {
            boids: [Boid::new(); M],
            _dv_coh: [[0.0; N]; M],
            _dv_sep: [[0.0; N]; M],
            _dv_ali: [[0.0; N]; M],
            _dv_bnd: [[0.0; N]; M],
        }
    }
    pub fn init(&mut self) {
        let mut rng = SmallRng::from_seed([0; 16]);
        for boid in self.boids.iter_mut() {
            for idx in 0..N {
                boid.position[idx] = (rng.gen::<u8>() as f32 - 128.0) / 128.0;
                boid.velocity[idx] = (rng.gen::<u8>() as f32 - 128.0) / 1280.0;
            }
        }
    }
    pub fn update(&mut self) {
        let mut dist = [0.0; M];
        let mut angle = [0.0; M];
        for (i, source) in self.boids.iter().enumerate() {
            for (j, target) in self.boids.iter().enumerate() {
                if i == j {
                    dist[j] = f32::MAX;
                    angle[j] = 0.0;
                } else {
                    dist[j] = calc_distance(source, target);
                    angle[j] = calc_angle(source, target);
                }
            }
            self._dv_coh[i] = calc_coherence(&source, &self.boids, dist, angle);
            self._dv_sep[i] = calc_separation(&source, &self.boids, dist, angle);
            self._dv_ali[i] = calc_alignment(&source, &self.boids, dist, angle);
            self._dv_bnd[i] = calc_boundary(&source, &self.boids, dist, angle);
        }
        for (idx, boid) in self.boids.iter_mut().enumerate() {
            boid.velocity = plus(
                boid.velocity,
                plus(
                    self._dv_coh[idx],
                    plus(
                        self._dv_sep[idx],
                        plus(self._dv_ali[idx], self._dv_bnd[idx]),
                    ),
                ),
            );
            let v_abs = norm(boid.velocity);
            if v_abs < MIN_VELOCITY {
                boid.velocity = divide(multiply(boid.velocity, MIN_VELOCITY), v_abs);
            } else if v_abs > MAX_VELOCITY {
                boid.velocity = divide(multiply(boid.velocity, MAX_VELOCITY), v_abs);
            }
            boid._prev_position = boid.position;
            boid.position = plus(boid.position, boid.velocity);
        }
    }
}

pub fn draw_boid<D>(boid: &Boid, idx: usize,display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Rgb565>,
{
    let prev_center = Point::new(
        (boid._prev_position[0] * SCALE) as i32 + CENTER_X,
        (boid._prev_position[1] * SCALE) as i32 + CENTER_Y,
    );
    let center = Point::new(
        (boid.position[0] * SCALE) as i32 + CENTER_X,
        (boid.position[1] * SCALE) as i32 + CENTER_Y,
    );
    let tip = Point::new(
        ((boid.position[0] + boid.velocity[0]) * SCALE) as i32 + CENTER_X,
        ((boid.position[1] + boid.velocity[1]) * SCALE) as i32 + CENTER_Y,
    );
    let color = if idx == 0 {Rgb565::RED} else {Rgb565::WHITE};
    Circle::new(prev_center, RAD)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(display)?;
    Circle::new(center, RAD)
        .into_styled(PrimitiveStyle::with_stroke(color, 1))
        .draw(display)?;
    Line::new(center, tip)
        .into_styled(PrimitiveStyle::with_stroke(color, 1))
        .draw(display)?;
    if idx == 0 {
        /* none */
    // Circle::new(center, (COHESION_DISTANCE * (SCALE as f32)) as u32)
    //     .into_styled(PrimitiveStyle::with_stroke(color, 1))
    //     .draw(display)?;
    }
    Ok(())
}

pub fn draw_boids<D>(boids: &Boids, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Rgb565>,
{
    for (idx, boid) in boids.boids.iter().enumerate() {
        draw_boid(boid, idx, display)?;
    }
    Ok(())
}
