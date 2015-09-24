fn sum_of_multiples_x_or_y(x: i32, y: i32, n: i32) -> i32 {
    
    let mut accum:i32 = 0;

    for num in 1..n {
        if num % x == 0 || num % y == 0 {
            accum += num;
        }
    }
    return accum;
}


fn sum_of_even_fibs_under_limit(n: i32) -> i32 {
    struct MyFibs {
        prev: i32,
        current: i32,
    }
    
    impl MyFibs {
        fn new(start: i32, current: i32)-> MyFibs {
            return MyFibs{prev: start, current: current}
        }
    }
    
    impl Iterator for MyFibs {
        type Item = i32;
    
        fn next(&mut self) -> Option<i32> {
            let temp = self.current;
            self.current = self.current + self.prev;
            self.prev = temp;
            Some(self.current)
        }
    }
    
    let fibs = MyFibs::new(1,1);
    fibs.take_while(|&el|el < n).filter(|&el|el % 2 == 0).fold(0,|acc, item| acc + item)
}

fn main() {
    println!("Euler solutions");
    println!("problem 1 result: {}", sum_of_multiples_x_or_y(3,5,1000));
    println!("problem 2 result: {}", sum_of_even_fibs_under_limit(4000000));
}
