pub struct IterBits {
    bits: u128,
    idx: u32,
}

impl Iterator for IterBits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < u128::BITS {
            let has = self.bits & 1 << self.idx == 1 << self.idx;
            let ret = has.then_some(self.idx);
            self.idx += 1;
            if ret.is_some() {
                return ret;
            }
        }

        None
    }
}

#[expect(non_camel_case_types)]
pub trait u128Ext {
    fn iter_bits(&self) -> IterBits;
}

impl u128Ext for u128 {
    fn iter_bits(&self) -> IterBits {
        IterBits {
            bits: *self,
            idx: 0,
        }
    }
}
