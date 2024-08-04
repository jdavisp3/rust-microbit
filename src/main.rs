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
        let display_state = display::init();

        let mut show_string = |s: &str| {
            for c in s.chars() {
                display.show(&mut timer, display::getchar(&display_state, c), 2000);
            }
            display.clear();
            timer.delay_ms(500_u32);
        };

        loop {
            show_string("MYRIAD GENE");
            show_string("❤");
        }
    }

    panic!("End");
}
