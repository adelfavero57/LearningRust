use std::io;

fn main() {
    println!("enter a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let input: i32 = input
        .trim()
        .parse()
        .expect("failed to convert to int");

    let fibbed = fib(input);
    println!("Fibonacci sequence starting with {input} is {fibbed}");
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        fib(n-2) + fib(n-1)
    }
}
