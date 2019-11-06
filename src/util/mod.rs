pub mod bit_vec;
pub mod factorization;

use factorization::Factorization;

/// Given a factorization f of n, returns the factorization
/// of the least multiplier that constructs n in Algorithm 3
pub fn least_multiplier(f: Factorization) -> usize {
    let p = f.greatest_prime_divisor();
    f.n / p
}

/// By trial division, determine if the value is prime or not
pub fn is_prime(n: usize) -> bool {
    for i in (2usize..).take_while(|i| i * i <= n) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {

    use super::*;
    
    #[test]
    fn test_trial_division() {
        let ns = vec![1, 10, 41, 121, 247];
        let bs = vec![true, false, true, false, false];
        for (&n, &b) in ns.iter().zip(bs.iter()) {
            assert_eq!(b, is_prime(n));
        }
    }
}