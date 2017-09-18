// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


mod age;
mod bidi;
mod category;
mod core;
mod name;
mod normal;


use writer::utils::clean_dir;


pub fn generate() {
    age::generate(&clean_dir("unic/ucd/age/tables"));
    bidi::generate(&clean_dir("unic/ucd/bidi/tables"));
    category::generate(&clean_dir("unic/ucd/category/tables"));
    core::generate(&clean_dir("unic/ucd/core/tables"));
    name::generate(&clean_dir("unic/ucd/name/tables"));
    normal::generate(&clean_dir("unic/ucd/normal/tables"));
}