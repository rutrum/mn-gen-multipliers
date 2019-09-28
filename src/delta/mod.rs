pub mod naive;

/// A walk from start to end inclusive.  Used to iterate
/// over rows in a multiplication table.
#[derive(Debug)]
pub struct RowWalk {
    start: usize,
    end: usize,
    step: usize,
}

impl RowWalk {
    /// Constructor from start to end with step size 1
    pub fn from(start: usize, end: usize) -> RowWalk {
        RowWalk {
            start,
            end,
            step: 1,
        }
    }

    /// Consumes a walk and changes step size
    pub fn by(self, step: usize) -> RowWalk {
        RowWalk {
            start: self.start,
            end: self.end,
            step,
        }
    }
}

impl Iterator for RowWalk {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.start > self.end {
            None
        } else {
            self.start += self.step;
            Some(self.start - self.step)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn walk_along_row_5() {
        let answer = (5..=50).step_by(5).collect::<Vec<usize>>();
        let walk = RowWalk::from(5, 50).by(5);
        assert_eq!(answer, walk.collect::<Vec<usize>>());
    }
}
