// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that we can define inherent methods on newtype enums that use
// an auto-ref'd receiver.

struct Foo(uint);

impl Foo {
    pub fn len(&self) -> uint { **self }
}

pub fn main() {
    let m = Foo(3);
    assert_eq!(m.len(), 3);
}