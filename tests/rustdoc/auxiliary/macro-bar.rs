//@ force-host
//@ no-prefer-dynamic
//@ compile-flags: --crate-type proc-macro

#![crate_type="proc-macro"]
#![crate_name = "bar"]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn a1(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
