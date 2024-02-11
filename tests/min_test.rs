#[default_const_functions::const_fn]
mod test {
    use default_const_functions::no_op;

    #[no_op]
    pub const fn test(x: i32) -> i32 {
        x
    }
    pub mut fn mut_test(x: i32) -> i32 {
        std::env::args();
        x
    }
}
use test::*;
#[test]
fn test_test() {
    let x = 5;
    assert_eq!(test(x), x)
}
#[test]
fn test_mut_test() {
    let x = 5;
    assert_eq!(mut_test(x), x)
}
