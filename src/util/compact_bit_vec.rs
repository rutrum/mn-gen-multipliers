pub struct CompactBitVec {
    bits: Vec<u8>,
}

impl CompactBitVec {
    pub fn new(size: usize) -> CompactBitVec {
        let len = (size + 7) / 8;
        CompactBitVec {
            bits: vec![0; len],
        }
    }

    // sets bit at loc
    pub fn set(&mut self, loc: usize) {
        self.bits[loc / 8] |= 1 << (loc % 8);
    }

    // pub fn has(&self, loc: usize) -> bool {
    //     self.bits[loc / 8] 
    // }

    // calculate hamming weight of bitvector
    pub fn count(&self) -> u32 {
        let mut count: u32 = 0;
        for n in &self.bits {
            
            count += (n & 0b0000_0001 > 0) as u32;
            count += (n & 0b0000_0010 > 0) as u32;
            count += (n & 0b0000_0100 > 0) as u32;
            count += (n & 0b0000_1000 > 0) as u32;
            count += (n & 0b0001_0000 > 0) as u32;
            count += (n & 0b0010_0000 > 0) as u32;
            count += (n & 0b0100_0000 > 0) as u32;
            count += (n & 0b1000_0000 > 0) as u32;
        }
        count
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn count_50_bits() {
        let mut bits = CompactBitVec::new(100);
        for i in (0..100).step_by(2) {
            bits.set(i);
            assert_eq!(bits.count(), 50);
        }
    }
}