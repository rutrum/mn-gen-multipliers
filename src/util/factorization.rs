/// Represents a pair of numbers.
/// Used for divisor pair of an integer or a
/// coordinate on a multiplication table.
#[derive(Debug, Clone, Copy)]
pub struct Pair(pub usize, pub usize);

/// Represents a given number's prime factorization.
/// The struct specifically stores a number and a
/// list of its divisor pairs.
#[derive(Debug)]
pub struct Factorization {
    pub n: usize,
    pub pairs: Vec<Pair>,
}

impl Factorization {
    /// Creates a new factorization given an integer n.  Automatically
    /// constructs the divisor pairs of n.
    pub fn new(n: usize) -> Factorization {
        Factorization {
            n,
            pairs: Factorization::gen_factor_pairs(n),
        }
    }

    /// Generates a list of factorizations in [a, b)
    pub fn in_range(a: usize, b: usize) -> Vec<Factorization> {
        let mut divisors: Vec<Vec<Pair>> = vec![];

        // Add trivial pairs
        for i in a..b {
            let pairs = vec![Pair(i, 1)];
            divisors.push(pairs);
        }

        // First seive through range to find all divisors
        for step in (2..).take_while(|i| i * i < b) {
            // Start is either the first integer greater than a that
            // is a multiple of step, or it is step^2 so pairs are unique
            // up to reordering
            let low = (a - 1) + step - (a - 1) % step;
            let start = std::cmp::max(low, step * step);

            for p in (start..b).step_by(step) {
                let index = p - a;
                divisors[index].push(Pair(p / step, step));
            }
        }

        // Now construct Factorizations and return
        let mut fs = vec![];
        for (i, d) in divisors.iter().enumerate() {
            fs.push(Factorization {
                n: a + i,
                pairs: d.clone(),
            })
        }
        fs
    }

    /// Generates all pairs of numbers (a,b) whose product is n,
    /// where a <= b
    fn gen_factor_pairs(n: usize) -> Vec<Pair> {
        let mut pairs = vec![Pair(n, 1)];
        for i in (2..).take_while(|i| i * i <= n) {
            if n % i == 0 {
                pairs.push(Pair(n / i, i));
            }
        }
        pairs
    }

    /// Returns the largest value in the lattice
    pub fn max_val(&self) -> usize {
        self.pairs.last().unwrap().0 * self.pairs.last().unwrap().1
    }

    pub fn kth_pair(&self, k: usize) -> Option<Pair> {
        match self.pairs.get(k) {
            Some(p) => Some(*p),
            None => None,
        }
    }

    pub fn first_pair(&self) -> Pair {
        self.pairs[0]
    }

    pub fn last_pair(&self) -> Pair {
        *self.pairs.last().unwrap()
    }

    /// Returns the total divisor pairs
    pub fn total_pairs(&self) -> usize {
        self.pairs.len()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    // Confirms that a factorization is valid
    #[test]
    fn validate_factorization() {
        let ns = vec![1, 10, 25, 41, 540, 100_000, 123_123, 5_040];
        for n in ns {
            let f = Factorization::new(n);
            valid_factorization(f);
        }
    }

    /// Tests that creating factorizations within an interval is valid.
    #[test]
    fn create_interval_of_factorizations() {
        let sizes = vec![(500, 600), (1, 5000), (1234, 12345)];
        for (a, b) in sizes {
            let fs = Factorization::in_range(a, b);
            assert_eq!(b - a, fs.len());
            for f in fs {
                valid_factorization(f);
            }
        }
    }

    /// Calls functions that validate divisor pairs
    fn valid_factorization(f: Factorization) {
        all_pairs_counted(&f);
        for pair in f.pairs {
            valid_pair(f.n, pair);
            ordered_pair(pair);
        }
    }

    /// Confirms that all possible divisor pairs have been accounted for
    /// by confirming number of possible divisor pairs.
    fn all_pairs_counted(f: &Factorization) {
        let n = f.n;
        let total_pairs = (1..=n) // equality needed for n = 1
            .take_while(|i| i * i <= n)
            .filter(|i| n % i == 0)
            .count();
        assert_eq!(total_pairs, f.total_pairs());
    }

    /// Asserts that the product of a pair is n
    fn valid_pair(n: usize, p: Pair) {
        assert_eq!(p.0 * p.1, n);
    }

    /// Asserts that Pair.0 is less than or equal to Pair.1
    fn ordered_pair(p: Pair) {
        assert!(p.0 >= p.1);
    }
}
