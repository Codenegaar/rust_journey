use codar::math::{simple, advanced};

#[test]
fn add_pow_basic() {
    let a:i64 = 4;
    let b:i64 = 4;
    let p:i64 = 2;

    let pow: i64 = advanced::power(b, p);
    let nums = vec![pow, a];
    let res = simple::add(&*nums);

    assert_eq!(res, 20);
}