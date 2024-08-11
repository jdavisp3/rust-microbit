#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::Timer};

mod display;

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display: Display = Display::new(board.display_pins);
        let display_state = display::init();
 
        loop {
            let s = "MYRIAD GENETICS ❤";
            let last_x = (s.chars().count() as i32 * 5) + 1;
            for x in -3..last_x {
                let mut screen: display::DisplayBuffer = [[0; 5]; 5];
                 for row in 0..5 {
                    for col in 0..5 {
                        let char_index = (x + col) / 5;
                        let mut bit: u8 = 0;
                        if char_index >= 0 && char_index < s.len() as i32 {
                            let col_offset = (x + col) % 5;
                            let c: char = s.chars().nth(char_index as usize).unwrap();
                            let display_c: display::DisplayBuffer = display::getchar(&display_state, c);
                            bit = display_c[row as usize][col_offset as usize];
                        }
                        screen[row as usize][col as usize] = bit;
                    }
                }
                display.show(&mut timer, screen, 1000);
            }
        }
    }

    panic!("End ❤");
}
