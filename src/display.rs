use heapless::FnvIndexMap;

pub struct DisplayState {
    charmap: FnvIndexMap<char, [[u8; 5]; 5], 32>,
}

const LETTER_A: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
];

const LETTER_D: [[u8; 5]; 5] = [
    [0, 1, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 0, 0],
];

const LETTER_E: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
];

const LETTER_G: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 1, 1, 0],
    [1, 0, 0, 1, 0],
    [0, 1, 1, 1, 0],
];

const LETTER_I: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 1, 1, 0],
];

const LETTER_L: [[u8; 5]; 5] = [
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
];

const LETTER_M: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 1, 0, 1, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];

const LETTER_N: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 1, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 1, 1],
    [1, 0, 0, 0, 1],
];

const LETTER_P: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
];

const LETTER_R: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
];

const LETTER_T: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

const LETTER_W: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0],
];

const LETTER_X: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1],
];

const LETTER_Y: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

const COLON: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

const HEART: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
];

const SPACE: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

pub fn init() -> DisplayState {
    let mut charmap = FnvIndexMap::<char, [[u8; 5]; 5], 32>::new();
    charmap.insert('A', LETTER_A).unwrap();
    charmap.insert('D', LETTER_D).unwrap();
    charmap.insert('E', LETTER_E).unwrap();
    charmap.insert('G', LETTER_G).unwrap();
    charmap.insert('I', LETTER_I).unwrap();
    charmap.insert('L', LETTER_L).unwrap();
    charmap.insert('M', LETTER_M).unwrap();
    charmap.insert('N', LETTER_N).unwrap();
    charmap.insert('P', LETTER_P).unwrap();
    charmap.insert('R', LETTER_R).unwrap();
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

pub fn getchar(state: &DisplayState, c: char) -> [[u8; 5]; 5] {
    match state.charmap.get(&c) {
        Some(&ch) => ch,
        None => panic!("Character not found"),
    }
}
