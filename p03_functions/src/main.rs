struct Student {
    age: u16,
    gender: char
}
impl Student {
    fn say_my_name(&self) {
        println!("My name is still unknown, but I'm {}", self.age);
    }
}

fn main() {
    let a: u64 = 6;
    let f_a = fact(a);
    println!("Fact. of 6 is: {}", f_a);

    let ali = Student {
        age: 21,
        gender: 'm'
    };
    ali.say_my_name();
}

fn fact(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * fact(n - 1),
    }
}
