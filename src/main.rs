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

        loop {
            display.show(&mut timer, display::getchar(&display_state, 'M'), 2000);
            display.show(&mut timer, display::getchar(&display_state, 'Y') , 2000);
            display.show(&mut timer, display::getchar(&display_state, 'R'), 2000);
            display.show(&mut timer, display::getchar(&display_state, 'I'), 2000);
            display.clear();
            display.show(&mut timer, display::getchar(&display_state, '❤'), 2000);
            timer.delay_ms(2500_u32);
        }
    }

    panic!("End");
}
