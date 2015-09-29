use problems::utils;
use problems::primes;


pub fn get_nth_prime(n: usize) -> usize {
    primes::PrimesGen::new().nth(n-1).unwrap()

}
