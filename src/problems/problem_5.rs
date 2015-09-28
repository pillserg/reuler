use problems::utils;


pub fn smallest_multiple(divisors: Vec<usize>) -> usize {
    let mut x = divisors.iter().fold(0, |acc, d|acc + d);
    let step = x;
    while !utils::is_divisors(x, &divisors) {
        x += step;
    }
    x
}
