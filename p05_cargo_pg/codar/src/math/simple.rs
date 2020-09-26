pub fn add<T>(args: &[T]) -> &T {
    let mut result: T = 0;
    for arg in args.iter() {
        result += arg;
    }

    result
}