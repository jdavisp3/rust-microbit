#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::Timer};

mod display_helper;

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer: Timer<microbit::pac::TIMER0> = Timer::new(board.TIMER0);
        let mut display: Display = Display::new(board.display_pins);
        let helper: display_helper::DisplayHelper = display_helper::init();
 
        loop {
            let s: &str = " MYRIAD GENETICS ❤";
            let last_x: usize = helper.get_scroll_width(s);
            for x in 0..last_x {
                let screen: display_helper::DisplayBuffer = helper.get_display_buffer_at_col(s, x);
                display.show(&mut timer, screen, 200);
            }
        }
    }

    panic!("End");
}
