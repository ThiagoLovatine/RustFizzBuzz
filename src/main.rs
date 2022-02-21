use std::io;

fn fizzbuzz(max: i32) {
    for x in 1..max {
        match (x % 3, x % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", x),
        }
    }
}

fn main() {
    let mut input = String::new();
    println!("Type a max number :");
    io::stdin().read_line(&mut input).expect("Invalid string");
    let max: i32 = input.trim().parse().expect("Invalid number");
    fizzbuzz(max);
}
