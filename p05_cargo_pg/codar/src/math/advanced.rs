/// Raises the parameter `b` to the power of `p`
///
/// # Examples
/// ```
/// use codar::math::advanced::power;
/// let b: i64 = 8;
/// let p: i64 = 2;
/// let res    = power(b, p);
/// assert_eq!(res, 64);
/// ```
pub fn power(b: i64, p: i64) -> i64 {
    let mut result: i64 = 1;
    for _n in 0..p {
        result *= b
    }

    result
}