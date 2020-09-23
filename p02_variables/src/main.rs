fn main() {
    //Default type is determined by context
    let a = 33; //Defaults to i32
    let b = 's'; //defaults to char
    let c = 3.14; //Defaults to f64

    //But can be manually assigned
    let d: u64 = 70;

    //Variables are immutable by deafault, unless specified
    let mut c: u32 = 80;
    c = 89;

    //And can be shadowed
    let a: bool = true;

    //To stop seeing warnings about a variable not being used, use _
    let _e = 85;
}
