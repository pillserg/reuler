use problems::utils;


pub fn largest_prime(n: usize) -> Option<usize> {
    for div in 2..n/2 {
        if n % div == 0 {
            let target = n/div;
            if utils::is_prime(target) {
                return Some(n/div)
            }
        }
    }
    None
}
