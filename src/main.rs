#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display: Display = Display::new(board.display_pins);

        #[allow(non_snake_case)]
        let letter_P = [
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_W = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
            [0, 1, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_L = [
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let colon = [
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_D = [
            [0, 1, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_X = [
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [1, 0, 0, 0, 1],
        ];
        loop {
            display.show(&mut timer, letter_P, 2000);
            display.show(&mut timer, letter_W, 2000);
            display.show(&mut timer, letter_L, 2000);
            display.show(&mut timer, colon, 2000);
            display.show(&mut timer, letter_P, 2000);
            display.show(&mut timer, letter_D, 2000);
            display.show(&mut timer, letter_X, 2000);
            display.clear();
            timer.delay_ms(250_u32);
        }
    }

    panic!("End");
}
