pub struct Sieve {
    divisors: Vec<usize>,
}

impl Sieve { 
    pub fn up_to(n: usize) -> Sieve {
        let mut divisors = vec![1; n+1];
        for i in 2..=n {
            if divisors[i] == 1 {
                for j in (i..=n).step_by(i) {
                    divisors[j] = i;
                }
            }
        }
        Sieve { divisors }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        self.divisors[n] == n
    }

    pub fn highest_prime_divisor(&self, n: usize) -> usize {
        self.divisors[n]
    }

    pub fn to_primes(&self) -> Vec<usize> {
        self.divisors.iter().enumerate()
            .filter(|(i, &d)| d == *i)
            .map(|(i, _)| i)
            .collect()
    }
}
