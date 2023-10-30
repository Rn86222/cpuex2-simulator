pub fn u8_to_i8(value: u8) -> i8 {
    if value <= i8::MAX as u8 {
        value as i8
    } else {
        (value as i16 - (u8::MAX as i16 + 1)) as i8
    }
}

pub fn i8_to_u8(value: i8) -> u8 {
    if value >= 0 {
        value as u8
    } else {
        (value as i16 + (u8::MAX as i16 + 1)) as u8
    }
}

pub fn u16_to_i16(value: u16) -> i16 {
    if value <= i16::MAX as u16 {
        value as i16
    } else {
        (value as i32 - (u16::MAX as i32 + 1)) as i16
    }
}

pub fn i16_to_u16(value: i16) -> u16 {
    if value >= 0 {
        value as u16
    } else {
        (value as i32 + (u16::MAX as i32 + 1)) as u16
    }
}

pub fn u32_to_i32(value: u32) -> i32 {
    if value <= i32::MAX as u32 {
        value as i32
    } else {
        (value as i64 - (u32::MAX as i64 + 1)) as i32
    }
}

pub fn i32_to_u32(value: i32) -> u32 {
    if value >= 0 {
        value as u32
    } else {
        (value as i64 + (u32::MAX as i64 + 1)) as u32
    }
}

pub fn sign_extention_i16(value: i16, before_bit: usize) -> i16 {
    if (value >> (before_bit - 1)) & 1 == 0 {
        value
    } else {
        let mut extention: i16 = 0;
        for i in 0..16 - before_bit {
            extention += 1 << (before_bit + i);
        }
        value | extention
    }
}

pub fn sign_extention_i32(value: i32, before_bit: usize) -> i32 {
    if (value >> (before_bit - 1)) & 1 == 0 {
        value
    } else {
        let mut extention: i32 = 0;
        for i in 0..32 - before_bit {
            extention += 1 << (before_bit + i);
        }
        value | extention
    }
}

#[allow(dead_code)]
pub const RED: &str = "31";
#[allow(dead_code)]
pub const BLUE: &str = "34";

pub fn colorize(text: &str, color: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", color, text)
}

#[allow(dead_code)]
pub fn colorized_println(text: &str, color: &str) {
    println!("{}", colorize(text, color));
}
