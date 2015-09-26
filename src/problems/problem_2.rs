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
    

pub fn sum_of_even_fibs_under_limit(n: i32) -> i32 {
    let fibs = MyFibs::new(1,1);
    fibs.take_while(|&el|el < n).filter(|&el|el % 2 == 0).fold(0,|acc, item| acc + item)
}

