use problems::utils;

pub fn get_nth_prime(n: usize) -> Option<usize> {
    println!("generating primes...");
    let primes = utils::sieve_primes(200000);
    match primes.get(n-1) {
        Some(i) => Some(*i),
        _ => None
    }
}
