fn main() {
    let a: u64 = 6;
    let f_a = fact(a);
    println!("Fact. of 6 is: {}", f_a);
}

fn fact(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * fact(n - 1),
    }
}
