mod util;
use util::factorization::Factorization;
use util::least_multiplier;
use util::sieve::Sieve;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let ub = 2_usize.pow(32);
    let mut covered = vec![false; ub + 1];
    let mut multipliers = Vec::new();

    let sieve = Sieve::up_to(ub);
    
    for n in (1..=ub) {
        if !covered[n] {

            let p = sieve.highest_prime_divisor(n);
            let m = n / p;

            multipliers.push((m, p));

            /*
            (1..=ub)
                .filter(|&c| sieve.is_prime(c))
                .map(|c| c * m)
                .take_while(|&x| x <= ub)
                .for_each(|x| {
                    covered[x] = true;
                });
            */
            
            (1..=ub/m)
                .filter(|&c| sieve.is_prime(c))
                .for_each(|x| {
                    covered[x * m] = true;
                });
        }
    }

    //println!("all covered: {:?}", covered.iter().skip(1).all(|x| *x));
    
    for (m, p) in multipliers { 
        println!("{} {}", m, p); 
    }
    
}

fn total_products_mod0(f: &Factorization) -> usize {
    let mut total = 0;
    let mut t = 1;
    for i in 1..f.last_pair().1 {
        if i == f.pairs[t].1 {
            t += 1;
        }
        let ub = f.pairs[t].0 * i;
        for p in ((i*i)..ub).step_by(i) {
            total += 1;
        }
    }
    total
}

fn write_out_multipliers() {
    for power in 0..=5 {
        let mut total_products = 0;

        let ub = 2_usize.pow(power);
        let mut m = vec![0; ub + 1];
        let mut multi = Vec::new();

        for i in (1..=ub).rev() {
            if m[i] == 0 {
                let f = Factorization::new(i);
                total_products += total_products_mod0(&f);
                let lm = least_multiplier(f);
                multi.push(lm);
                for j in (lm..=ub).step_by(lm) {
                    if lm == least_multiplier(Factorization::new(j)) {
                        m[j] = i;
                    }
                }
            }
        }

        println!("Total products under 2^{}: {}", power, total_products);

        let mut file = File::create(format!("multipliers2pow{}.txt", power)).unwrap();
        file.write_all(format!("{:?}", multi).as_bytes()).unwrap(); 
    }
}
