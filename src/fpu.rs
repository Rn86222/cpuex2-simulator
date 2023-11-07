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

    fn get_sign(&self) -> i32 {
        if self.value & 0x80000000 != 0 {
            -1
        } else {
            1
        }
    }

    fn get_exp(&self) -> i32 {
        let mut result = self.value;
        let mut exp = result & 0x7f800000;
        exp >>= 23;
        exp as i32 - 127
    }

    fn get_fraction(&self) -> u32 {
        let mut result = self.value;
        let mut fraction = result & 0x7fffff;
        fraction |= 0x800000;
        fraction
    }

    fn get_all(&self) -> (i32, i32, u32) {
        (self.get_sign(), self.get_exp(), self.get_fraction())
    }

    fn gets(&self) -> (u32, u32, u32) {
        (
            to_n_bits_u32((self.value & 0x80000000) >> 31, 1),
            to_n_bits_u32((self.value & 0x7f800000) >> 23, 8),
            to_n_bits_u32(self.value & 0x7fffff, 23),
        )
    }
}

fn to_n_bits_u32(num: u32, n: u32) -> u32 {
    let mut n = 1 << n;
    n -= 1;
    num & n
}

fn to_n_bits_u64(num: u64, n: u32) -> u64 {
    let mut n = 1 << n;
    n -= 1;
    num & n
}

// impl Display for FloatingPoint {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut result = self.value;
//         let mut exp = 0;
//         while result & 0x80000000 == 0 {
//             result <<= 1;
//             exp -= 1;
//         }
//         result <<= 1;
//         exp -= 1;
//         result >>= 9;
//         result &= 0x7fffff;
//         result |= (exp as u32 + 127) << 23;
//         write!(f, "{:x}", result)
//     }
// }

impl Debug for FloatingPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (sign, exp, fraction) = self.get_all();
        write!(f, "{} x 1.{:>032b} x 2^{}", sign, fraction, exp)
    }
}

impl Add for FloatingPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let (s1, e1, m1) = self.gets();
        let (s2, e2, m2) = other.gets();
        let (m1a, e1a) = if e1 == 0 {
            (to_n_bits_u32(m1, 25), 1)
        } else {
            (to_n_bits_u32(m1 | 0x800000, 25), e1)
        };
        let (m2a, e2a) = if e2 == 0 {
            (to_n_bits_u32(m2, 25), 1)
        } else {
            (to_n_bits_u32(m2 | 0x800000, 25), e2)
        };
        let (ce, tde) = if e1a > e2a {
            (0 as u32, to_n_bits_u32(e1a - e2a, 8))
        } else {
            (1 as u32, to_n_bits_u32(e2a - e1a, 8))
        };
        let de = if tde >> 5 != 0 {
            31
        } else {
            to_n_bits_u32(tde, 5)
        };
        let sel = if de == 0 {
            if m1a > m2a {
                0
            } else {
                1
            }
        } else {
            ce
        };
        let (ms, mi, es, ss) = if sel == 0 {
            (m1a, m2a, e1a, s1)
        } else {
            (m2a, m1a, e2a, s2)
        };
        let mie = to_n_bits_u64((mi as u64) << 31, 56);
        let mia = to_n_bits_u64(mie >> (de as u64), 56);
        let tstck: u32 = if to_n_bits_u64(mia, 29) != 0 { 1 } else { 0 };
        let mye = if s1 == s2 {
            to_n_bits_u64(((ms as u64) << 2) + (mia >> 29), 27)
        } else {
            to_n_bits_u64(((ms as u64) << 2) - (mia >> 29), 27)
        };
        let esi = to_n_bits_u32(es + 1, 8);
        let (eyd, myd, stck) = if mye & (1 << 26) != 0 {
            if esi == 255 {
                (255, 1 << 25, 0)
            } else {
                (esi, to_n_bits_u64(mye >> 1, 27), tstck | (mye & 1) as u32)
            }
        } else {
            (es, mye, tstck)
        };
        let se = to_n_bits_u64(myd, 26).leading_zeros() - 38;
        let eyf = eyd as i64 - se as i64;
        let (myf, eyr) = if eyf > 0 {
            (to_n_bits_u64(myd << se, 56), (eyf & 0xFF) as u32)
        } else {
            (to_n_bits_u64(myd << ((eyd & 31) - 1), 56), 0)
        };
        let myr = if (myf & 0b10 != 0 && myf & 0b1 == 0 && stck == 0 && myf & 0b100 != 0)
            || (myf & 0b10 != 0 && myf & 0b1 == 0 && s1 == s2 && stck == 1)
            || (myf & 0b10 != 0 && myf & 0b1 != 0)
        {
            to_n_bits_u64(to_n_bits_u64(myf >> 2, 25) + 1, 25)
        } else {
            to_n_bits_u64(myf >> 2, 25)
        };
        let eyri = to_n_bits_u32(eyr + 1, 8);
        let (ey, my) = if (myr >> 24) & 1 != 0 {
            (eyri, 0)
        } else if to_n_bits_u64(myr, 24) == 0 {
            (0, 0)
        } else {
            (eyr, to_n_bits_u64(myr, 23))
        };
        let sy = if ey == 0 && my == 0 { s1 & s2 } else { ss };
        let nzm1 = if to_n_bits_u32(m1, 23) != 0 { 1 } else { 0 };
        let nzm2 = if to_n_bits_u32(m2, 23) != 0 { 1 } else { 0 };
        let y = if e1 == 255 && e2 != 255 {
            (s1 << 31) + (255 << 23) + (nzm1 << 22) + to_n_bits_u32(m1, 22)
        } else if e1 != 255 && e2 == 255 {
            (s2 << 31) + (255 << 23) + (nzm2 << 22) + to_n_bits_u32(m2, 22)
        } else if e1 == 255 && e2 == 255 && nzm1 == 1 {
            (s1 << 31) + (255 << 23) + (1 << 22) + to_n_bits_u32(m1, 22)
        } else if e1 == 255 && e2 == 255 && nzm2 == 1 {
            (s2 << 31) + (255 << 23) + (1 << 22) + to_n_bits_u32(m2, 22)
        } else if e1 == 255 && e2 == 255 && s1 == s2 {
            (s1 << 31) + (255 << 23)
        } else if e1 == 255 && e2 == 255 {
            (1 << 31) + (255 << 23) + (1 << 22)
        } else {
            (sy << 31) + (ey << 23) + (my as u32)
        };

        let _ovf = if e1 == 255 && e2 == 255 {
            0
        } else if ((mye >> 26) & 1 == 1 && esi == 255) || (myr >> 24) & 1 == 1 && eyri == 255 {
            1
        } else {
            0
        };
        FloatingPoint { value: y }
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
