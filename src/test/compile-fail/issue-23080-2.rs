// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-tidy-linelength

#![feature(optin_builtin_traits)]

unsafe auto trait Trait {
//~^ ERROR E0380
    type Output;
}

fn call_method<T: Trait>(x: T) {}

fn main() {
    call_method(());
    //~^^^^^^^^^^^^^^^^^^^^^^^ ERROR overflow evaluating the requirement
}
