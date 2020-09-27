/// Adds a list of numbers and returns the result
/// # Examples
/// ```
/// use codar::math::simple::add;
/// let nums = vec![12, 14, 16];
/// let res  = add(&nums);
/// assert_eq!(res, 42);
/// ```
pub fn add(args: &[i64]) -> i64 {
    let mut result = 0;
    for arg in args.iter() {
        result += arg;
    }

    result
}