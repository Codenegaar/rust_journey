use super::*;

#[test]
fn simple_add_basic() {
    let numbers = vec![10, 20, 30, 22];
    let sum = simple::add(&numbers);
    assert_eq!(sum, 82);
}