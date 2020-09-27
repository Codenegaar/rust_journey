pub fn power(b: i64, p: i64) -> i64 {
    let mut result: i64 = 1;
    for _n in 0..p {
        result *= b
    }

    result
}