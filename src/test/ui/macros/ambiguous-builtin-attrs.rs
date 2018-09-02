#![feature(decl_macro)]

macro repr() {}

#[repr(C)] //~ ERROR `repr` is ambiguous
struct S;
#[cfg_attr(all(), repr(C))] //~ ERROR `repr` is ambiguous
struct SCond;

macro cfg() {}

#[cfg(all())] //~ ERROR `cfg` is ambiguous
struct A;
#[cfg(any())] // ERROR FIXME
struct A;

macro cfg_attr() {}

#[cfg_attr(all(), cold)] // ERROR FIXME
fn g() {}
#[cfg_attr(any(), cold)] // ERROR FIXME
fn h() {}

macro derive() {}

#[derive(Clone)] // ERROR FIXME
struct B;

macro test() {}

#[test] // ERROR FIXME
fn test() {}

macro bench() {}

#[bench] // ERROR FIXME
fn bench() {}

macro_rules! inline { () => () }

#[inline] //~ ERROR `inline` is ambiguous
fn f() {}
#[cfg_attr(all(), inline)] //~ ERROR `inline` is ambiguous
fn f_cond() {}

fn main() {}
