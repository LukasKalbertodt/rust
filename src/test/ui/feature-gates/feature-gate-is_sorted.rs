// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    assert!([1, 2, 2, 9].iter().is_sorted());
    //^ ERROR: use of unstable library feature 'is_sorted'
    assert!(![-2i32, -1, 0, 3].iter().is_sorted_by_key(|n| n.abs()));
    //^ ERROR: use of unstable library feature 'is_sorted'
}