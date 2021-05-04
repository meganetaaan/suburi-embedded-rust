#![no_std]

use embedded_graphics::{
    egcircle, egrectangle, egtext,
    fonts::Font8x16,
    image::{Image, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
    primitive_style, text_style,
};

const CASE_COLOR: Rgb565 = Rgb565::WHITE;
const BUTTON_COLOR: Rgb565 = Rgb565::BLUE;
const BG_COLOR: Rgb565 = Rgb565::BLACK;

pub struct WioSplash<'a> {
    text_color: Rgb565,
    image: ImageRawLE<'a, Rgb565>
}

impl<'a> WioSplash<'a> {
    pub fn new(
        text_color: Rgb565,
        image: ImageRawLE<'a, Rgb565>,
    ) -> Self {
        Self { text_color, image }
    }

    fn draw_case<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Rgb565>,
    {
        egrectangle!(
            top_left = (20, 20),
            bottom_right = (300, 220),
            style = primitive_style!(
                stroke_width = 5,
                stroke_color = CASE_COLOR,
                fill_color = BG_COLOR,
            )
        )
        .draw(display)?;

        egrectangle!(
            top_left = (20, 180),
            bottom_right = (300, 220),
            style = primitive_style!(fill_color = CASE_COLOR)
        )
        .draw(display)?;

        for i in 0..4 {
            egrectangle!(
                top_left = (40 + i * 15, 190),
                bottom_right = (45 + i * 15, 210),
                style = primitive_style!(fill_color = BG_COLOR)
            )
            .draw(display)?;
        }

        Ok(())
    }

    fn draw_buttons<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Rgb565>,
    {
        for i in 0..3 {
            egrectangle!(
                top_left = (40 + i * 60, 15),
                bottom_right = (80 + i * 60, 20),
                style = primitive_style!(fill_color = BUTTON_COLOR)
            )
            .draw(display)?;
        }

        egcircle!(
            center = (260, 180),
            radius = 20,
            style = primitive_style!(fill_color = BUTTON_COLOR)
        )
        .draw(display)
    }

    fn draw_image<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Rgb565>,
    {
        use core::convert::TryFrom;
        let (w, h) = display.size().into();
        let (iw, ih) = (self.image.width(), self.image.height());
        let (x, y) = (
            i32::try_from(w / 2 - iw / 2).unwrap(),
            i32::try_from(h / 2 - ih / 2).unwrap(),
        );
        let top_left = Point::new(x, y);
        let image = Image::new(&self.image, top_left);
        image.draw(display)
    }
}

impl<'a> Drawable<Rgb565> for WioSplash<'a> {
    fn draw<D>(self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Rgb565>,
    {
        self.draw_case(display)?;
        self.draw_buttons(display)?;
        self.draw_image(display)?;

        egtext!(
            text = "Booting Wio Terminal...",
            top_left = (30, 30),
            style = text_style!(
                font = Font8x16,
                text_color = self.text_color,
            )
        )
        .draw(display)
    }
}