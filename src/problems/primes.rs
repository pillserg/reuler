use std::collections::HashMap;


pub fn is_prime(n: usize) -> bool {
    if n == 4 {
        return false
    }
    for div in 2..n/2 {
        if n % div == 0 {
            return false
        }
    }
    true
}


pub fn sieve_primes(n: usize) -> Vec<usize> {
    // n - sieve size
    let mut sieve: HashMap<usize, bool> = HashMap::with_capacity(n);

    for x in (2..n) {
        sieve.insert(x, true);
    }
    
    let mut p: Option<usize> = Some(2);

    while let Some(_p) = p {
        for x in (_p+_p..n).step_by(_p) {
            sieve.insert(x, false);
        }
        p = (0..n).position(|x:usize| x > _p && sieve[&x]);
    }
    let mut ret = sieve.iter().filter(|&(&p, &x)| x).map(|x|*x.0).collect::<Vec<usize>>();
    ret.sort();
    ret
}


pub struct PrimesGen {
    iteration: usize,
    current: usize,
}


impl PrimesGen {
    pub fn new()-> PrimesGen {
        return PrimesGen { current: 2, iteration: 0 }
    }
}

impl Iterator for PrimesGen {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.iteration += 1;

        if self.iteration == 1 {
            return Some(self.current)
        }

        self.current = self.current + 1;
        while !is_prime(self.current) {
            self.current += 1;
        }
        Some(self.current)
    }
}
