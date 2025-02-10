//@ aux-build:macro-bar.rs

#![crate_name = "foo"]

extern crate bar;

//@ has foo/attr.new_a1.html '//a/@href' 'attr.a1.html'

/// Link to [`a1`].
pub use bar::a1 as new_a1;

pub use bar::a1;
