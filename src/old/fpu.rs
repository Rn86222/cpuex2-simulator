impl Mul for FloatingPoint {
    type Output = Self;
    // fn mul(self, other: Self) -> Self {
    //     let (s1, e1, m1) = self.gets();
    //     let (s2, e2, m2) = other.gets();
    //     let (m1a, e1a) = if e1 == 0 {
    //         (to_n_bits_u32(m1, 25), 1)
    //     } else {
    //         (to_n_bits_u32(m1 | 0x800000, 25), e1)
    //     };
    //     let (m2a, e2a) = if e2 == 0 {
    //         (to_n_bits_u32(m2, 25), 1)
    //     } else {
    //         (to_n_bits_u32(m2 | 0x800000, 25), e2)
    //     };
    //     let myi = (m1a as u64) * (m2a as u64);
    //     let mei = e1a + e2a - 127;
    //     let stub = myi & ((1 << 20) - 1);
    //     let myri = myi >> 20;
    //     let myr = if stub != 0 { myri | 1 } else { myri };
    //     let se = 64 - myr.leading_zeros();
    //     let (myra, mera) = if myr >= (1 << 27) {
    //         if to_n_bits_u64(myri, (se - 27) as u32) != 0 {
    //             ((myri >> (se - 27)) | 1, mei + (se - 27))
    //         } else {
    //             (myri >> (se - 27), mei + (se - 27))
    //         }
    //     } else {
    //         (myri, mei)
    //     };
    //     let grs = to_n_bits_u64(myra, 3);
    //     let mys = if grs >= 0b100 { myra + (1 << 3) } else { myra };
    //     let ses = 64 - mys.leading_zeros();
    //     let (mysa, ey) = if mys >= (1 << 27) {
    //         if to_n_bits_u64(mys, (ses - 27) as u32) != 0 {
    //             ((mys >> (ses - 27)) | 1, mera + (ses - 27))
    //         } else {
    //             (mys >> (ses - 27), mera + (ses - 27))
    //         }
    //     } else {
    //         (mys, mera)
    //     };
    //     let my = to_n_bits_u64(mysa >> 3, 23);
    //     let sy = s1 ^ s2;
    //     let y = (sy << 31) + (ey << 23) + (my as u32);
    //     FloatingPoint { value: y }
    // }
}
