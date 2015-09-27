pub fn is_prime(n: usize) -> bool {
    for div in 2..n/2 {
        if n % div == 0 {
            return false
        }
    }
    true
}
