use std::string::ToString;
use std::num::{ One, Zero };
use std::collections::HashMap;
pub use problems::primes::{ is_prime, sieve_primes };

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

