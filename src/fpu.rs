use std::cmp::*;
use std::fmt::{Debug, Display};
use std::ops::*;

#[derive(Copy, Clone)]
pub struct FloatingPoint {
    value: u32,
}

impl FloatingPoint {
    pub fn new(value: u32) -> Self {
        FloatingPoint { value }
    }

    pub fn new_f32(value_f32: f32) -> Self {
        FloatingPoint {
            value: value_f32.to_bits(),
        }
    }
}

impl Display for FloatingPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = self.value;
        let mut exp = 0;
        while result & 0x80000000 == 0 {
            result <<= 1;
            exp -= 1;
        }
        result <<= 1;
        exp -= 1;
        result >>= 9;
        result &= 0x7fffff;
        result |= (exp as u32 + 127) << 23;
        write!(f, "{:x}", result)
    }
}

impl Debug for FloatingPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", f32::from_bits(self.value))
    }
}

impl Add for FloatingPoint {
    type Output = Self;
    fn add(self, _: Self) -> Self {
        unimplemented!();
        // let result = self.value;
        // FloatingPoint { value: result }
    }
}

impl Sub for FloatingPoint {
    type Output = Self;
    fn sub(self, _: Self) -> Self {
        unimplemented!();
        // let result = self.value;
        // FloatingPoint { value: result }
    }
}

impl Mul for FloatingPoint {
    type Output = Self;
    fn mul(self, _: Self) -> Self {
        unimplemented!();
        // let result = self.value;
        // FloatingPoint { value: result }
    }
}

impl Div for FloatingPoint {
    type Output = Self;
    fn div(self, _: Self) -> Self {
        unimplemented!();
        // let result = self.value;
        // FloatingPoint { value: result }
    }
}

impl Neg for FloatingPoint {
    type Output = Self;
    fn neg(self) -> Self {
        let mut result = self.value;
        result ^= 0x80000000;
        FloatingPoint { value: result }
    }
}

impl PartialEq for FloatingPoint {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for FloatingPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut result = self.value;
        let mut result2 = other.value;
        if result == 0 {
            result = 1;
        }
        if result2 == 0 {
            result2 = 1;
        }
        if result > result2 {
            Some(Ordering::Greater)
        } else if result < result2 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Eq for FloatingPoint {}

impl Ord for FloatingPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut result = self.value;
        let mut result2 = other.value;
        if result == 0 {
            result = 1;
        }
        if result2 == 0 {
            result2 = 1;
        }
        if result > result2 {
            Ordering::Greater
        } else if result < result2 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
