#![allow(unused)]
#![feature(custom_inner_attributes, proc_macro_hygiene, prelude_import)]
#![::default_const_functions::const_fn]

use default_const_functions::no_op;

#[no_op]
pub const fn test(x: i32) -> i32 {
    x
}
fn mut_test(x: i32) -> i32 {
    x
}

#[test]
fn test_test() {}
