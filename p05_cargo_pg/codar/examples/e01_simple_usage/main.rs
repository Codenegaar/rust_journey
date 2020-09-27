use std::io;

use codar::math::{simple::add, advanced::power};

fn main() {
    let mut res: i64 = 0;
    let mut num: i64 = 0;
    let mut opr: char;

    loop {
        println!("*** Current result is: {} ***", res);
        let mut input = String::new();

        println!("Enter a number (0 to exit): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");
        num = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue
            }
        };
        if num == 0 { break }

        input = String::new();
        println!("Enter an operator (+, ^): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");
        opr = input.chars().next().unwrap();

        match (opr) {
            '+' => {
                res = add(&(vec![res, num]));
            },
            '^' => {
                res = power(res, num);
            },
            _ => {
                println!("Invalid operator, supported operators are: +, ^");
            },
        }
    }
}