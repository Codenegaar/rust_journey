pub fn add(args: &[i32]) -> i32 {
    let mut result = 0;
    for arg in args.iter() {
        result += arg;
    }

    result
}