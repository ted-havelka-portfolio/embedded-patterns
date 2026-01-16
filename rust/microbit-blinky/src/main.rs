#![no_std]
#![no_main]

// For this source code see:
// *  https://github.com/charlesmuchene/minimal-rust-microbit-starter/blob/main/src/main.rs
// For article to learn and develop this stub see:
// *  https://medium.com/@charlesmuchene/micro-bits-d-9f273c9462dd

use microbit as _;
use panic_halt as _;

use microbit::{
    board::Board,
    // hal::prelude::{OutputPin, PinState},
    hal::{
        prelude::{OutputPin, PinState, _embedded_hal_blocking_delay_DelayMs},
        timer::Timer,
    },
};

#[cortex_m_rt::entry]
fn start_here() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let blink_delay_ms = 300_u32;

    board.display_pins.col3.set_state(PinState::Low).unwrap();

    loop {
        board.display_pins.row3.set_state(PinState::High).unwrap();
        timer.delay_ms(blink_delay_ms);
        board.display_pins.row3.set_state(PinState::Low).unwrap();
        timer.delay_ms(blink_delay_ms);
    }
}
