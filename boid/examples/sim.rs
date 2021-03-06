use std::{thread, time};
use boid::*;
use boid::Boids;
use embedded_graphics::prelude::*;
use embedded_graphics::{egrectangle, primitive_style, pixelcolor::Rgb565};
use embedded_graphics_simulator::*;

static SCREEN_WIDTH: u16 = 480;
static SCREEN_HEIGHT: u16 = 360;

fn clear_screen<T: embedded_graphics::DrawTarget<Rgb565>>(
    display: &mut T,
) -> Result<(), T::Error> {
    egrectangle!(
        top_left = (0, 0),
        bottom_right = ((SCREEN_WIDTH - 1).into(), (SCREEN_HEIGHT - 1).into()),
        style = primitive_style!(fill_color = boid::BG_COLOR)
    )
    .draw(display)
}

fn main() {
    let mut boids: Boids = Boids::new();
    let mut display: SimulatorDisplay<Rgb565> =
      SimulatorDisplay::new(Size::new(SCREEN_WIDTH.into(), SCREEN_HEIGHT.into()));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Boids", &output_settings);

    clear_screen(&mut display).unwrap();
    boids.init();
    boids.update();
    let mut boids_renderer = BoidRenderer::new();
    boids_renderer.draw(&mut display, &boids).unwrap();
    window.show_static(&display);

    let interval = time::Duration::from_millis(10);
    'running: loop {
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running;
        }
        boids.update();
        clear_screen(&mut display).unwrap();
        boids_renderer.draw(&mut display, &boids).unwrap();
        window.update(&display);
        thread::sleep(interval);
    }
}
