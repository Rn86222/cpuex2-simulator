impl Mul for FloatingPoint {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let (s1, e1, m1) = self.get_1_8_23_bits();
        let (s2, e2, m2) = other.get_1_8_23_bits();
        if e1 == 0 || e2 == 0 {
            return FloatingPoint { value: 0 };
        }
        let (h1, h2) = (m1 >> 11, m2 >> 11);
        let (l1, l2) = (m1 & 0x7ff, m2 & 0x7ff);
        let h1i = h1 | 0x1000;
        let h2i = h2 | 0x1000;
        let h1h2 = (h1i * h2i) as u64;
        let h1l2 = (h1i * l2) as u64;
        let l1h2 = (l1 * h2i) as u64;
        let l1l2 = (l1 * l2) as u64;
        let m1m2 = (h1h2 << 22) + (h1l2 << 11) + (l1h2 << 11) + l1l2;
        let se = 63 - m1m2.leading_zeros();
        let myi = m1m2 & ((1 << se) - 1);
        let my = if se < 23 {
            myi << (23 - se)
        } else {
            myi >> (se - 23)
        };
        let ei = match se {
            47.. => e1 + e2 + 1,
            46 => e1 + e2,
            _ => panic!(),
        };
        let ey = if ei < 127 { 0 } else { ei - 127 };
        let sy = s1 ^ s2;
        let y = (sy << 31) + (ey << 23) + (my as u32);
        FloatingPoint { value: y }
    }
}
