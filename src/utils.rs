// Converts the raw value (2^N) from the read() method into a keypad digit. This will be
//      0..9    digits
//      -1      *
//      -2      #

pub(crate) fn convert(value: u16) -> i16 {
    match value {
        KEY_1 => 1,
        KEY_4 => 4,
        KEY_7 => 7,
        KEY_STAR => -1,
        KEY_2 => 2,
        KEY_5 => 5,
        KEY_8 => 8,
        KEY_0 => 0,
        KEY_3 => 3,
        KEY_6 => 6,
        KEY_9 => 9,
        KEY_HASH => -2,
        KEY_A => -3,
        KEY_B => -4,
        KEY_C => -5,
        KEY_D => -6,
        _ => -10
    }
}

const KEY_1: u16 = 1;
const KEY_4: u16 = 1 << 1;
const KEY_7: u16 = 1 << 2;
const KEY_STAR: u16 = 1 << 3;
const KEY_2: u16 = 1 << 4;
const KEY_5: u16 = 1 << 5;
const KEY_8: u16 = 1 << 6;
const KEY_0: u16 = 1 << 7;
const KEY_3: u16 = 1 << 8;
const KEY_6: u16 = 1 << 9;
const KEY_9: u16 = 1 << 10;
const KEY_HASH: u16 = 1 << 11;
const KEY_A: u16 = 1 << 12;
const KEY_B: u16 = 1 << 13;
const KEY_C: u16 = 1 << 14;
const KEY_D: u16 = 1 << 15;