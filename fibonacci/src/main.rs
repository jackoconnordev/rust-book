use std::io::{self, Write};

fn main() {
    print!("Print the n-th fibonacci number: ");
    io::stdout().flush().expect("Failed to flush to stdout"); // unwrap ignores errors
    
    let result: u64;
    loop {
        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line :(");

        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue
            }
        };

        result = fib(n);
        break;
    }
    println!("The n-th fibonacci number is {result}");
}

fn fib(n: u64) -> u64 {
    let (mut old, mut new) = (0, 1);
    if n == 0 { return old }
    else if n == 1 { return new }
    else {
        for _ in 0..n-1 {
            (old, new) = (new, old + new);
        }
        return new
    }
}
