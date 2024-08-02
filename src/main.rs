#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};

mod display;

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display: Display = Display::new(board.display_pins);

        loop {
            display.show(&mut timer, display::LETTER_P, 2000);
            display.show(&mut timer, display::LETTER_W, 2000);
            display.show(&mut timer, display::LETTER_L, 2000);
            display.show(&mut timer, display::COLON, 2000);
            display.show(&mut timer, display::LETTER_P, 2000);
            display.show(&mut timer, display::LETTER_D, 2000);
            display.show(&mut timer, display::LETTER_X, 2000);
            display.clear();
            display.show(&mut timer, display::HEART, 2000);
            timer.delay_ms(2500_u32);
        }
    }

    panic!("End");
}
