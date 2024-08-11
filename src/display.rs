use heapless::FnvIndexMap;

pub type DisplayBuffer = [[u8; 5]; 5];

pub struct DisplayState {
    charmap: FnvIndexMap<char, DisplayBuffer, 32>,
}

const LETTER_A: DisplayBuffer = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
];

const LETTER_C: DisplayBuffer = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
];

const LETTER_D: DisplayBuffer = [
    [0, 1, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 0, 0],
];

const LETTER_E: DisplayBuffer = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
];

const LETTER_G: DisplayBuffer = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 1, 1, 0],
    [1, 0, 0, 1, 0],
    [0, 1, 1, 1, 0],
];

const LETTER_I: DisplayBuffer = [
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 1, 1, 0],
];

const LETTER_L: DisplayBuffer = [
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
];

const LETTER_M: DisplayBuffer = [
    [1, 0, 0, 0, 1],
    [1, 1, 0, 1, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];

const LETTER_N: DisplayBuffer = [
    [1, 0, 0, 0, 1],
    [1, 1, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 1, 1],
    [1, 0, 0, 0, 1],
];

const LETTER_P: DisplayBuffer = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
];

const LETTER_R: DisplayBuffer = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
];

const LETTER_S: DisplayBuffer = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 1, 1, 1, 0],
];

const LETTER_T: DisplayBuffer = [
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

const LETTER_W: DisplayBuffer = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0],
];

const LETTER_X: DisplayBuffer = [
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1],
];

const LETTER_Y: DisplayBuffer = [
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

const COLON: DisplayBuffer = [
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

const HEART: DisplayBuffer = [
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
];

const SPACE: DisplayBuffer = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

pub fn init() -> DisplayState {
    let mut charmap = FnvIndexMap::<char, DisplayBuffer, 32>::new();
    charmap.insert('A', LETTER_A).unwrap();
    charmap.insert('C', LETTER_C).unwrap();
    charmap.insert('D', LETTER_D).unwrap();
    charmap.insert('E', LETTER_E).unwrap();
    charmap.insert('G', LETTER_G).unwrap();
    charmap.insert('I', LETTER_I).unwrap();
    charmap.insert('L', LETTER_L).unwrap();
    charmap.insert('M', LETTER_M).unwrap();
    charmap.insert('N', LETTER_N).unwrap();
    charmap.insert('P', LETTER_P).unwrap();
    charmap.insert('R', LETTER_R).unwrap();
    charmap.insert('S', LETTER_S).unwrap();
    charmap.insert('T', LETTER_T).unwrap();
    charmap.insert('W', LETTER_W).unwrap();
    charmap.insert('X', LETTER_X).unwrap();
    charmap.insert('Y', LETTER_Y).unwrap();
    charmap.insert(':', COLON).unwrap();
    charmap.insert('❤', HEART).unwrap();
    charmap.insert(' ', SPACE).unwrap();
    DisplayState {
        charmap,
    }
}

pub fn getchar(state: &DisplayState, c: char) -> DisplayBuffer {
    match state.charmap.get(&c) {
        Some(&ch) => ch,
        None => panic!("Character not found"),
    }
}
