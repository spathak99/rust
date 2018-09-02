// compile-flags:--test

#![feature(decl_macro, test)]

extern crate test;

macro test() {}

#[test] //~ ERROR `test` is ambiguous
fn test() {}

macro bench() {}

#[bench] //~ ERROR `bench` is ambiguous
fn bench(b: &mut test::Bencher) {}

fn main() {}
