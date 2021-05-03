//! 6-2 LEDとボタン/GPIOのサンプルコードです。
//! ボタン1 (一番右のボタン) を押している間、ユーザーLEDが点灯します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-2-led_and_button
//! ```

#![no_std]
#![no_main]
#![allow(dead_code)]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::hal::gpio::*;
use wio::pac::Peripherals;
use wio::prelude::*; // 主要な構造体やトレイトをインポートする

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut pins = wio::Pins::new(peripherals.PORT);

    let mut led = Led::new(pins.user_led, &mut pins.port);
    let mut button1 = Button1::new(pins.button1, &mut pins.port);

    loop {
        if button1.is_pressed() {
            led.turn_on();
        } else {
            led.turn_off();
        }
    }
}

struct Button1 {
    pin: Pc26<Input<Floating>>
}

impl Button1 {
    fn new(pin: Pc26<Input<Floating>>, port: &mut Port) -> Button1 {
        Button1 {
            pin: pin.into_floating_input(port)
        }
    }
    fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }
    fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}

struct Led {
    pin: Pa15<Output<PushPull>>
}

impl Led {
    fn new(pin: Pa15<Input<Floating>>, port: &mut Port) -> Led {
        Led {
            pin: pin.into_push_pull_output(port)
        }
    }

    fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }

    fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }

    fn toggle(&mut self) {
        self.pin.toggle();
    }
}