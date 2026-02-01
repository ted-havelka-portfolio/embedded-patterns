#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::peripherals::SERIAL0;
use embassy_nrf::{bind_interrupts, uarte};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    SERIAL0 => uarte::InterruptHandler<SERIAL0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;

    // From https://tech.microbit.org/hardware/schematic/
    // RX P0.6, TX P1.8
    // From https://github.com/embassy-rs/embassy/blob/main/embassy-nrf/src/uarte.rs#L162
    // RX pin is parameter 2, TX pin is parameter 3
    // let mut uart = uarte::Uarte::new(p.SERIAL0, p.P1_00, p.P1_01, Irqs, config);
    let mut uart = uarte::Uarte::new(p.SERIAL0, p.P1_08, p.P0_06, Irqs, config);

    info!("uarte initialized!");

    // Message must be in SRAM
    let mut buf = [0; 8];
    buf.copy_from_slice(b"Hello!\r\n");

    unwrap!(uart.write(&buf).await);
    info!("wrote hello in uart!");

    loop {
        info!("reading...");
        unwrap!(uart.read(&mut buf).await);
        info!("writing...");
        unwrap!(uart.write(&buf).await);
    }
}
