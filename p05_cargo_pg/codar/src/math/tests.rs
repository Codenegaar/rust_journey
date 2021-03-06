use super::*;

#[test]
fn simple_add_basic() {
    let numbers = vec![10, 20, 30, 22];
    let sum = simple::add(&numbers);
    assert_eq!(sum, 82);
}

#[test]
fn advanced_power_basic() {
    let b: i64 = 3;
    let p: i64 = 3;
    let res: i64 = advanced::power(b, p);

    assert_eq!(res, 27);
}