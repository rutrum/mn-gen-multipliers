use crate::util::{
    bit_vec::BitVec,
    factorization::{Factorization, Pair},
};
use super::RowWalk;

#[derive(Debug)]
pub struct NaiveIterator {
    k: usize,
    x: usize,
    y: usize,
    f: Factorization,
    h: usize,
}

impl NaiveIterator {
    /// Creates a new naive iterator over the given integer
    pub fn over_n(n: usize) -> NaiveIterator {
        let f = Factorization::new(n);
        NaiveIterator::over(f)
    }

    /// Creates a new naive iterator over the given factorization
    pub fn over(f: Factorization) -> NaiveIterator {
        let h = f.last_pair().1;
        NaiveIterator {
            k: 1,
            x: 0,
            y: 1,
            f,
            h,
        }
    }

    fn current_pair(&self) -> Option<Pair> {
        self.f.kth_pair(self.k)
    }
}

impl Iterator for NaiveIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match self.current_pair() {
            Some(p) => {
                self.x += 1;

                // Check if need to step up one row
                if self.x >= p.0 {
                    self.y += 1;
                    self.x = self.y;

                    // Check if need to advance to next pair
                    if self.y >= p.1 {
                        self.k += 1;

                        // Check if went over pairs
                        if p.1 >= self.h {
                            return None;
                        }
                    }
                }

                Some(self.x * self.y)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_over_45() {
        let answer: Vec<usize> = (1..=14)
            .chain((4..=28).step_by(2))
            .chain((9..=24).step_by(3))
            .chain((16..=32).step_by(4))
            .collect();
        let delta_iter = NaiveIterator::over_n(45);
        assert_eq!(answer, delta_iter.collect::<Vec<usize>>());
    }

    #[test]
    fn iter_over_12() {
        let answer = vec![1, 2, 3, 4, 5, 4, 6];
        let delta_iter = NaiveIterator::over_n(12);
        assert_eq!(answer, delta_iter.collect::<Vec<usize>>());
    }

    #[test]
    fn iter_over_prime() {
        let mut delta_iter = NaiveIterator::over_n(13);
        assert_eq!(None, delta_iter.next());
    }
}
