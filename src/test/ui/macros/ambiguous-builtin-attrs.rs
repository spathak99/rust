#![feature(decl_macro)] //~ ERROR `feature` is ambiguous

macro feature() {}

macro repr() {}

#[repr(C)] //~ ERROR `repr` is ambiguous
struct S;
#[cfg_attr(all(), repr(C))] //~ ERROR `repr` is ambiguous
struct SCond;

macro cfg() {} //~ ERROR name `cfg` is reserved in macro namespace

#[cfg(all())] //~ ERROR `cfg` is ambiguous
struct A;
#[cfg(any())] // ERROR FIXME
struct A;

macro cfg_attr() {} //~ ERROR name `cfg_attr` is reserved in macro namespace

#[cfg_attr(all(), cold)] // ERROR FIXME
fn g() {}
#[cfg_attr(any(), cold)] // ERROR FIXME
fn h() {}

macro derive() {} //~ ERROR name `derive` is reserved in macro namespace

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

fn non_macro_expanded_location<#[inline] T>() { //~ ERROR `inline` is ambiguous
    match 0u8 {
        #[repr(C)] //~ ERROR `repr` is ambiguous
        _ => {}
    }
}

fn main() {}
