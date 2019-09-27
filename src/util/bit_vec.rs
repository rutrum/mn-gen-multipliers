pub struct BitVec {
    bits: Vec<bool>,
}

impl BitVec {
    /// Creates new bit vector of given size, initialized to false
    pub fn new(size: usize) -> BitVec {
        BitVec {
            bits: vec![false; size],
        }
    }

    /// Sets the given location of bit vec to true
    pub fn set(&mut self, loc: usize) {
        self.bits[loc] = true;
    }

    /// Returns the total number of trues in the vector
    pub fn count(&self) -> usize {
        self.bits.iter().filter(|x| **x).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_many() {
        for size in vec![100, 1_000, 10_000] {
            for step in vec![2, 3, 30] {
                let mut bits = BitVec::new(size);
                for i in (0..size).step_by(step) {
                    bits.set(i);
                }
                let actual_count = (size + step - 1) / step;
                assert_eq!(actual_count, bits.count());
            }
        }
    }
}
