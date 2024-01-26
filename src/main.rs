use std::io::{self, Write};

use euler::get_solution;

fn main() {
    let mut buf = String::new();
    let (number, solution) = loop {
        buf.clear();
        let mut number = None;
        while number.is_none() {
            print!("Solution number: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buf).unwrap();
            number = buf.trim().parse().ok();
        }

        let number = number.unwrap();
        match get_solution(number) {
            Some(solution) => break (number, solution),
            None => println!("No solution for {number}"),
        }
    };

    println!("Showing solution for {number}:");
    println!("{}", solution());
}
