#![no_std]
use core::f32::consts::PI;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::pixelcolor::RgbColor;
use embedded_graphics::prelude::Drawable;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::primitives::Triangle;
use embedded_graphics::style::PrimitiveStyle;
use embedded_graphics::DrawTarget;
use micromath::F32Ext;
use rand::prelude::*;
const COHESION_FORCE: f32 = 0.005;
const SEPARATION_FORCE: f32 = 0.1;
const ALIGNMENT_FORCE: f32 = 0.08;
const BOUNDARY_FORCE: f32 = 0.01;
const COHESION_DISTANCE: f32 = 0.35;
const SEPARATION_DISTANCE: f32 = 0.1;
const ALIGNMENT_DISTANCE: f32 = 0.1;
const COHESION_ANGLE: f32 = PI / 2.0;
const SEPARATION_ANGLE: f32 = PI / 2.0;
const ALIGNMENT_ANGLE: f32 = PI / 3.0;
const MIN_VELOCITY: f32 = 0.005;
const MAX_VELOCITY: f32 = 0.03;
const N: usize = 3;
const M: usize = 30;
const WING_WIDTH: f32 = 5.0;
pub const BG_COLOR: Rgb565 = Rgb565::BLACK;
pub const BOID_COLOR: Rgb565 = Rgb565::WHITE;
pub const SPECIAL_BOID_COLOR: Rgb565 = Rgb565::RED;

#[derive(Debug, Clone, Copy)]
pub struct Boid {
    position: [f32; N],
    _prev_position: [f32; N],
    velocity: [f32; N],
    _prev_velocity: [f32; N],
    _prev_points: Option<(Point, Point, Point)>,
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
        multiply(minus(average, source.position), COHESION_FORCE)
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
            _prev_velocity: [0.0; N],
            _prev_points: None,
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
        assert_eq!((N == 2 || N == 3), true);
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
            boid._prev_position = boid.position;
            boid._prev_velocity = boid.velocity;
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
            boid.position = plus(boid.position, boid.velocity);
        }
    }
}

fn calc_size(position: [f32; N]) -> f32 {
    if N == 3 {
        let z = position[2]; // -1.0 < z < 1.0
        let w = (WING_WIDTH as f32) * (z + 1.0);
        w
    } else {
        WING_WIDTH
    }
}
struct DrawContext {
    center_x: i32,
    center_y: i32,
    scale: f32,
}

fn calc_points(position: [f32; N], velocity: [f32; N], ctx: &DrawContext) -> (Point, Point, Point) {
    let center_x = ctx.center_x;
    let center_y = ctx.center_y;
    let scale = ctx.scale;
    let size = calc_size(position);
    let n = (velocity[0] * velocity[0] + velocity[1] * velocity[1]).sqrt();
    let v = norm(velocity);
    let z = (MAX_VELOCITY - velocity[2].abs()) / MAX_VELOCITY; // 0.005 ~ 0.03
    let s = 1.0 + v / MAX_VELOCITY;
    let aprox_zoom = if position[2] < -1.4 {
        0.1
    } else {
        position[2] + 1.5
    };
    let vel_x = (velocity[0] / n) * size;
    let vel_y = (velocity[1] / n) * size;
    let start_x = (position[0] * aprox_zoom * scale) as i32;
    let start_y = (position[1] * aprox_zoom * scale) as i32;
    let top = Point::new(
        start_x + (vel_x * s * z) as i32 + center_x,
        start_y + (vel_y * s * z) as i32 + center_y,
    );
    let right = Point::new(
        start_x + (vel_y / s) as i32 + center_x,
        start_y - (vel_x / s) as i32 + center_y,
    );
    let left = Point::new(
        start_x - (vel_y / s) as i32 + center_x,
        start_y + (vel_x / s) as i32 + center_y,
    );
    (top, right, left)
}

fn clear_prev_boid<D>(
    boid: &Boid,
    _idx: usize,
    display: &mut D,
    _ctx: &DrawContext,
) -> Result<(), D::Error>
where
    D: DrawTarget<Rgb565>,
{
    let (top, right, left) = boid._prev_points.unwrap();
    if right == left {
        Rectangle::new(top, top)
            .into_styled(PrimitiveStyle::with_fill(BG_COLOR))
            .draw(display)?
    } else {
        Triangle::new(top, right, left)
            .into_styled(PrimitiveStyle::with_stroke(BG_COLOR, 1))
            .draw(display)?;
    }
    Ok(())
}
fn draw_boid<D>(
    boid: &mut Boid,
    _idx: usize,
    display: &mut D,
    ctx: &DrawContext,
) -> Result<(), D::Error>
where
    D: DrawTarget<Rgb565>,
{
    let (top, right, left) = calc_points(boid.position, boid.velocity, ctx);
    boid._prev_points = Some((top, right, left));
    let color: Rgb565 = if _idx == 0 {
        SPECIAL_BOID_COLOR
    } else {
        BOID_COLOR
    };
    if right == left {
        Rectangle::new(top, top)
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(display)?
    } else {
        Triangle::new(top, right, left)
            .into_styled(PrimitiveStyle::with_stroke(color, 1))
            .draw(display)?;
    }
    Ok(())
}

pub fn draw_boids<D>(boids: &mut Boids, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Rgb565>,
{
    let ctx: DrawContext = {
        let (w, h) = display.size().into();
        let center_x: i32 = (w / 2) as i32;
        let center_y: i32 = (h / 2) as i32;
        let scale: f32 = if center_x < center_y {
            (center_x as f32) * 0.5
        } else {
            (center_y as f32) * 0.5
        };
        DrawContext {
            center_x,
            center_y,
            scale,
        }
    };
    for (idx, boid) in boids.boids.iter().enumerate() {
        if boid._prev_points != None {
            clear_prev_boid(boid, idx, display, &ctx)?;
        }
    }
    for (idx, boid) in boids.boids.iter_mut().enumerate() {
        draw_boid(boid, idx, display, &ctx)?;
    }
    Ok(())
}
