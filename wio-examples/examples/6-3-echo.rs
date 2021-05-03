//! 6-3 シリアル入出力/UARTのサンプルコードです。
//! ホストPCのシリアルターミナルに入力した内容をそのまま出力します
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-3-echo
//! ```

#![no_std]
#![no_main]

use wio_terminal as wio;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use core::panic::PanicInfo;
use cortex_m::interrupt::{self, Mutex};
// use panic_halt as _;
use wio::hal::clock::GenericClockController;
use wio::hal::gpio::*;
use wio::hal::sercom::*;
use wio::pac::Peripherals;
use wio::prelude::*;
use wio::{entry, Pins, Sets};

static UART: Mutex<RefCell<Option<UART2<Sercom2Pad1<Pb27<PfC>>, Sercom2Pad0<Pb26<PfC>>, (), ()>>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    // クロックを初期化する
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    // UARTドライバオブジェクトを初期化する
    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    interrupt::free(|cs| UART.borrow(cs).replace(Some(serial)));
    // hello world を出力する
    interrupt::free(|cs| {
        if let Some(ref mut serial) = UART.borrow(cs).borrow_mut().deref_mut() {
            writeln!(serial, "hello world").unwrap();
        }
    });
    // let none: Option<usize> = None;
    // none.unwrap();
    loop {
        interrupt::free(|cs| {
            if let Some(ref mut serial) = UART.borrow(cs).borrow_mut().deref_mut() {
                // データを 1 ワード受信すると if ブロック内に入る
                if let Ok(c) = nb::block!(serial.read()) {
                    // 受信したデータをそのまま送信する
                    nb::block!(serial.write(c)).unwrap();
                }
            }
        });
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::free(|cs| {
        if let Some(ref mut serial) = UART.borrow(cs).borrow_mut().deref_mut() {
            // パニックハンドラ内でさらにパニックしないように、unwrap() しない
            let _ = writeln!(serial, "panic: {}", info);
        }
    });
    loop {}
}
