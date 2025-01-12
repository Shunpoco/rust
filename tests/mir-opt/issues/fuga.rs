// skip-filecheck
#![feature(never_patterns)]
#![allow(incomplete_features)]

fn main() {
    match () {
        (!|!) if true => {}
        (!|!) if true => {}
    }
}