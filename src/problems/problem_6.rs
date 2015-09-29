use num::bigint::{BigUint, ToBigUint};

// overflows
pub fn _diff_sum_of_squares_square_of_sums(n: u64) -> u64{
    let sum_of_squares: u64 = (1..n).sum();
    let sum: u64 = (1..n).sum();
    sum_of_squares - sum * sum
}

pub fn diff_sum_of_squares_square_of_sums(n: usize) -> BigUint {
    let mut sum_of_squares = 0.to_biguint().unwrap(); 
    let mut sum = 0;
    
    for x in (1..n){
        let _x = &x.to_biguint().unwrap();
        sum_of_squares = _x * _x + sum_of_squares;
        sum += x;
    }
    let sum = sum.to_biguint().unwrap();
    sum.to_biguint().unwrap() * sum.to_biguint().unwrap() - sum_of_squares
}
