extern crate docopt;
extern crate rustc_serialize;
extern crate reuler;
use reuler::problems;
use docopt::Docopt;


static USAGE: &'static str = "
Usage:
    reuler [options]

Options: 
    --help              Show this message
    -p <problem>, --problem <problem>   Problem number [default: 0]
";

#[derive(RustcDecodable)]
struct CliArgs {
    flag_problem: u32,
}


fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d|d.decode::<CliArgs>())
        .unwrap_or_else(|e|e.exit());

    println!("Euler solutions");

    match args.flag_problem {
        0 => println!("Please specify problem number"),
        1 => println!("problem 1 result: {}", problems::problem_1::sum_of_multiples_x_or_y(3,5,1000)),
        2 => println!("problem 2 result: {}", problems::problem_2::sum_of_even_fibs_under_limit(4000000)),
        3 => println!("problem 3 result: {:?}", problems::problem_3::largest_prime(600851475143).unwrap()),
        4 => println!("problem 4 result: {}", problems::problem_4::largest_pallindrome_product(999, 999)),
        _ => println!("Problem Not implemented"),
    }
}
