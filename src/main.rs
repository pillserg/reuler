fn sum_of_multiples_x_or_y(x: i32, y: i32, n: i32) -> i32 {
    
    let mut accum:i32 = 0;

    for num in 1..n {
        if num % x == 0 || num % y == 0 {
            accum += num;
        }
    }
    return accum;
}

fn main() {
    println!("Hello, world!");
    println!("{}", sum_of_multiples_x_or_y(3,5,10));
    println!("{}", sum_of_multiples_x_or_y(3,5,1000));
}
