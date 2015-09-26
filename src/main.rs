extern crate reuler;
use reuler::problems;

fn main() {
    println!("Euler solutions");
    println!("problem 1 result: {}", problems::problem_1::sum_of_multiples_x_or_y(3,5,1000));
    println!("problem 2 result: {}", problems::problem_2::sum_of_even_fibs_under_limit(4000000));
}
