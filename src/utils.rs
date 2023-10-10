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

pub fn u32_to_i32(value: u32) -> i32 {
    if value <= i32::MAX as u32 {
        value as i32
    } else {
        (value as i64 - (u32::MAX as i64 + 1)) as i32
    }
}
