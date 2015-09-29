use std::string::ToString;
use std::num::{ One, Zero };
use std::collections::HashMap;


pub fn is_prime(n: usize) -> bool {
    for div in 2..n/2 {
        if n % div == 0 {
            return false
        }
    }
    true
}

pub fn is_pallindrom<T: ToString + One +  Zero>(n: T) -> bool {
    // lets pretend only ascii exists in the world. rusts unicode handling and iteration is a
    // little cryptic )
    let s_n: String = n.to_string();
    let bytes = s_n.as_bytes();
    let len = bytes.len();
    for (idx, c) in bytes.iter().enumerate() {
        if bytes[len - idx - 1] != *c {
            return false
        }
    }
    true
}

pub fn is_divisors(x: usize, divisors: &Vec<usize>) -> bool {
    for d in divisors.iter() {
        if x % d != 0 {
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




