use std::string::ToString;
use std::num::{ One, Zero };

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
