#![no_main]
#![no_std]

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use microbit::hal::uarte::{self, Baudrate, Parity};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use core::fmt::Write;

use microbit::{
// QUESTION [ ] why no need to call out `microbit::Board` here?
//   Is `Board` a feature or module at the same level as `hal`?
    hal::Rng,
};

use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    // (2) Hardware random number generator
    let mut rng = Rng::new(board.RNG);

    // (1) UART related
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    serial.write(b'X').unwrap();
    serial.flush().unwrap();
    write!(serial, "\nRandom number: {}\n\r", rng.random_u8()).unwrap();
    serial.flush().unwrap();
    rprintln!("{}", rng.random_u8());

    loop {
        wfi();
    }
}
