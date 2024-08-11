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

/*         let mut show_string = |s: &str| {
            for c in s.chars() {
                display.show(&mut timer, display::getchar(&display_state, c), 1000);
            }
            display.clear();
            timer.delay_ms(500_u32);
        };
 */
         let mut scroll_string = |s: &str| {
            let last_x = s.len() as i32 * 5 + 1;
            for x in -5..last_x {
                let mut screen: display::DisplayBuffer = [[0; 5]; 5];
                for row in 0..5 {
                    let char_index = (x + row) / 5;
                    for col in 0..5 {
                        let mut bit: u8 = 0;
                        if char_index >= 0 && char_index < s.len() as i32 {
                            bit = display::getchar(&display_state, s.chars().nth(char_index as usize).unwrap())[row as usize][col as usize];
                        }
                        screen[row as usize][col as usize] = bit;
                    }
                }
                display.show(&mut timer, screen, 200);
            }
            timer.delay_ms(500_u32);
        };
 
        loop {
            scroll_string("MYRIAD GENETICS ❤");
        }
    }

    panic!("End");
}
