use heapless::FnvIndexMap;

pub type DisplayBuffer = [[u8; 5]; 5];

#[derive(Clone, Copy)]
pub struct CharBuffer {
    pub buffer: DisplayBuffer,
    pub start_col: usize,
    pub end_col: usize,
}

impl CharBuffer {
    pub fn width(&self) -> usize {
        return self.end_col - self.start_col + 1;
    }
}

pub struct DisplayHelper {
    charmap: FnvIndexMap<char, CharBuffer, 32>,
}


impl DisplayHelper {
    pub fn getchar(&self, c: char) -> CharBuffer {
        match self.charmap.get(&c) {
            Some(&ch) => ch,
            None => panic!("Character not found"),
        }
    }

    pub fn get_scroll_width(&self, s: &str) -> usize {
        let mut width: usize = 0;
        for c in s.chars() {
            width += self.getchar(c).width();
        }
        return width + s.chars().count();
    }

    pub fn get_display_buffer_at_col(&self, s: &str, mut start_col: usize) -> DisplayBuffer {
        let mut screen: DisplayBuffer = BLANK;
        start_col = start_col.div_euclid(self.get_scroll_width(s));
        return screen;
    }
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

pub const BLANK: DisplayBuffer = SPACE;


fn char_to_buffer(ch: DisplayBuffer) -> CharBuffer {
    let mut start_col = 0;
    for col in 0..5 {
        let mut empty = true;
        for row in 0..5 {
            if ch[row][col] == 1 {
                empty = false;
                break;
            }
        }
        if !empty {
            start_col = col;
            break;
        }
    }
    let mut end_col = 4;
    for col in (start_col..5).rev() {
        let mut empty = true;
        for row in 0..5 {
            if ch[row][col] == 1 {
                empty = false;
                break;
            }
        }
        if !empty {
            end_col = col;
            break;
        }
    }
    let cb: CharBuffer = CharBuffer {
        buffer: ch,
        start_col: start_col,
        end_col: end_col,
    };
    return cb;
}

pub fn init() -> DisplayHelper {
    let mut charmap = FnvIndexMap::<char, CharBuffer, 32>::new();
    let _ = charmap.insert('A', char_to_buffer(LETTER_A));
    let _ = charmap.insert('C', char_to_buffer(LETTER_C));
    let _ = charmap.insert('D', char_to_buffer(LETTER_D));
    let _ = charmap.insert('E', char_to_buffer(LETTER_E));
    let _ = charmap.insert('G', char_to_buffer(LETTER_G));
    let _ = charmap.insert('I', char_to_buffer(LETTER_I));
    let _ = charmap.insert('L', char_to_buffer(LETTER_L));
    let _ = charmap.insert('M', char_to_buffer(LETTER_M));
    let _ = charmap.insert('N', char_to_buffer(LETTER_N));
    let _ = charmap.insert('P', char_to_buffer(LETTER_P));
    let _ = charmap.insert('R', char_to_buffer(LETTER_R));
    let _ = charmap.insert('S', char_to_buffer(LETTER_S));
    let _ = charmap.insert('T', char_to_buffer(LETTER_T));
    let _ = charmap.insert('W', char_to_buffer(LETTER_W));
    let _ = charmap.insert('X', char_to_buffer(LETTER_X));
    let _ = charmap.insert('Y', char_to_buffer(LETTER_Y));
    let _ = charmap.insert(':', char_to_buffer(COLON));
    let _ = charmap.insert('❤', char_to_buffer(HEART));
    let _ = charmap.insert(' ', char_to_buffer(SPACE));
    DisplayHelper {
        charmap,
    }
}
