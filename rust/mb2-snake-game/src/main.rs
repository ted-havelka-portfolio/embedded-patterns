// This work copied from
// https://docs.rust-embedded.org/discovery/microbit/11-snake-game/game-logic.html

#![no_main]
#![no_std]

mod game;
mod control;
mod display;

use cortex_m_rt::entry;
use microbit::{
    Board,
    hal::{
        prelude::*,
        Rng,
        timer::Timer
    },
    // display::blocking::Display
    display::nonblocking::{BitImage, GreyscaleImage}
};

use rtt_target::rtt_init_print;
use panic_rtt_target as _;

use crate::control::{init_buttons, get_turn};
use crate::display::{clear_display, display_image, init_display};
use crate::game::{Game, GameStatus, Turn};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut rng = Rng::new(board.RNG);
    let mut game = Game::new(rng.random_u32());
    // let mut display = Display::new(board.display_pins);

    init_buttons(board.GPIOTE, board.buttons);
    init_display(board.TIMER1, board.display_pins);

    loop {
        loop {  // Game loop
            // Game matrix parameters are brightness for snake head, tail and food
            let image = GreyscaleImage::new(&game.game_matrix(9, 9, 9));
            // display.show(&mut timer, image, game.step_len_ms());
            display_image(&image);
            match game.status {
                GameStatus::Ongoing => game.step(get_turn(true)),
                _ => {
                    for _ in 0..3 {
                        // display.clear();
                        clear_display();
                        timer.delay_ms(200u32);
                        // display.show(&mut timer, image, 200);
                        display_image(&image);
                    }
                    // display.clear();
                    // display.show(&mut timer, game.score_matrix(), 1000);
                    clear_display();
                    display_image(&BitImage::new(&game.score_matrix()));
                    break
                }
            }
        }
        game.reset();
    }
}
