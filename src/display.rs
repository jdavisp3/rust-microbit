use heapless::FnvIndexMap;
use microbit::{display::blocking::Display, hal::Timer};

pub struct DisplayState {
    charmap: FnvIndexMap<char, [[u8; 5]; 5], 16>,
}

pub const LETTER_D: [[u8; 5]; 5] = [
    [0, 1, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 0, 0],
];

pub const LETTER_L: [[u8; 5]; 5] = [
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
];

pub const LETTER_M: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 1, 0, 1, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];

pub const LETTER_P: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
];

pub const LETTER_W: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0],
];

pub const LETTER_X: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1],
];

pub const LETTER_Y: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

pub const COLON: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

pub const HEART: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
];

pub fn init() -> DisplayState {
    let mut charmap = FnvIndexMap::<char, [[u8; 5]; 5], 16>::new();
    charmap.insert('D', LETTER_D).unwrap();
    charmap.insert('L', LETTER_L).unwrap();
    charmap.insert('M', LETTER_M).unwrap();
    charmap.insert('P', LETTER_P).unwrap();
    charmap.insert('W', LETTER_W).unwrap();
    charmap.insert('X', LETTER_X).unwrap();
    charmap.insert('Y', LETTER_Y).unwrap();
    charmap.insert(':', COLON).unwrap();
    charmap.insert('❤', HEART).unwrap();
    DisplayState {
        charmap,
    }
}

pub fn getchar(state: &DisplayState, c: char) -> Option<&[[u8; 5]; 5]> {
    state.charmap.get(&c)
}
