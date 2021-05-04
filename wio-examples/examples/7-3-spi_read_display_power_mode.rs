//! 7-3 LCD/SPIのサンプルコードです。
//! SPIでILI9341からDisplay Power Modeを取得します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 7-3-spi_read_display_power_mode
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use core::fmt::Write;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::gpio::*;
use wio::hal::sercom::*;
use wio::hal::hal::spi;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

#[entry]
fn main() -> ! {
    // 1. ドライバの初期化
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut sets = wio::Pins::new(peripherals.PORT).split();
    // UARTドライバオブジェクトを初期化する
    let mut serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // TODO: SPIドライバオブジェクトを初期化する
    let gclk0 = &clocks.gclk0();
    let mut spi: SPIMaster7<
        Sercom7Pad2<Pb18<PfD>>,
        Sercom7Pad3<Pb19<PfD>>,
        Sercom7Pad1<Pb20<PfD>>,
    > = SPIMaster7::new(
        &clocks.sercom7_core(&gclk0).unwrap(),
        8.mhz(),
        spi::MODE_0,
        peripherals.SERCOM7,
        &mut peripherals.MCLK,
        (
            sets.display.miso.into_pad(&mut sets.port),
            sets.display.mosi.into_pad(&mut sets.port),
            sets.display.sck.into_pad(&mut sets.port),
        ),
    );

    // TODO: その他の制御信号を出力に設定する
    let mut cs = sets.display.cs.into_push_pull_output(&mut sets.port);
    let mut dc = sets.display.dc.into_push_pull_output(&mut sets.port);
    let mut reset = sets.display.reset.into_push_pull_output(&mut sets.port);

    // TODO: 2. ILI9341のハードウェアリセット
    reset.set_low().unwrap();
    delay.delay_us(100u16);
    reset.set_high().unwrap();
    delay.delay_ms(120u16);

    // TODO: 3. Read Display Power Modeコマンド（0x0A）の送信
    cs.set_low().unwrap();
    dc.set_low().unwrap();
    spi.write(&[0x0A]).unwrap();

    // TODO: 4. データ出力の読み込み
    let mut args = [0x00]; // dummy
    dc.set_high().unwrap();
    let mode = spi.transfer(&mut args).unwrap();
    cs.set_high().unwrap();

    // TODO: 5. 読み込んだデータをシリアルに出力
    writeln!(&mut serial, "display power mode = 0x{:<02X}", mode[0]).unwrap();

    loop {}
}
