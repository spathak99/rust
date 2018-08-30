// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass

use std::ops::Add;

trait Positioned<S> {
  fn SetX(&mut self, _: S);
  fn X(&self) -> S;
}

trait Movable<S: Add<Output=S>>: Positioned<S> {
  fn translate(&mut self, dx: S) {
    let x = self.X() + dx;
    self.SetX(x);
  }
}

struct Point { x: isize, y: isize }

impl Positioned<isize> for Point {
    fn SetX(&mut self, x: isize) {
        self.x = x;
    }
    fn X(&self) -> isize {
        self.x
    }
}

impl Movable<isize> for Point {}

pub fn main() {
    let mut p = Point{ x: 1, y: 2};
    p.translate(3);
    assert_eq!(p.X(), 4);
}
