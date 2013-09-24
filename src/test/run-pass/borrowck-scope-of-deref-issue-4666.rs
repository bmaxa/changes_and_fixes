// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Tests that the scope of the pointer returned from `get()` is
// limited to the deref operation itself, and does not infect the
// block as a whole.

struct Box {
    x: uint
}

impl Box {
    fn get<'a>(&'a self) -> &'a uint {
        &self.x
    }
    fn set(&mut self, x: uint) {
        self.x = x;
    }
}

fn fun1() {
    // in the past, borrow checker behaved differently when
    // init and decl of `v` were distinct
    let v;
    let mut box = Box {x: 0};
    box.set(22);
    v = *box.get();
    box.set(v+1);
    assert_eq!(23, *box.get());
}

fn fun2() {
    let mut box = Box {x: 0};
    box.set(22);
    let v = *box.get();
    box.set(v+1);
    assert_eq!(23, *box.get());
}

pub fn main() {
    fun1();
    fun2();
}