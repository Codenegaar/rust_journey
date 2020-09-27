pub fn add(args: &[i64]) -> i64 {
    let mut result = 0;
    for arg in args.iter() {
        result += arg;
    }

    result
}