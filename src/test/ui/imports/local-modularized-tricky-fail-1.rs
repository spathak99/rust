// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(decl_macro)]

macro_rules! define_exported { () => {
    #[macro_export]
    macro_rules! exported {
        () => ()
    }
}}
macro_rules! define_panic { () => {
    #[macro_export]
    macro_rules! panic {
        () => ()
    }
}}
macro_rules! define_include { () => {
    #[macro_export]
    macro_rules! include {
        () => ()
    }
}}

use inner1::*;

mod inner1 {
    pub macro exported() {}
}

exported!(); //~ ERROR `exported` is ambiguous

mod inner2 {
    define_exported!();
}

fn main() {
    panic!(); //~ ERROR `panic` is ambiguous
              //~^ ERROR `panic` is ambiguous
}

mod inner3 {
    define_panic!();
}

mod inner4 {
    define_include!();
}

include!(); //~ ERROR `include` is ambiguous
