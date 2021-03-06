// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// edition:2015
// aux-build:edition-kw-macro-2015.rs
// compile-pass

#![allow(keyword_idents)]

#[macro_use]
extern crate edition_kw_macro_2015;

mod one_async {
    produces_async! {} // OK
}
mod two_async {
    produces_async_raw! {} // OK
}

fn main() {}
