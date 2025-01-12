// skip-filecheck
// EMIT_MIR hoge.assign_twice.PreCodegen.after.mir
//@ compile-flags: -Z mir-opt-level=3 -Z span_free_formats

#![feature(never_patterns)]
#![feature(if_let_guard)]
#![allow(incomplete_features)]

fn split_last(_: &()) -> Option<(&i32, &i32)> {
    None
}

fn assign_twice() {
    loop {
        match () {            
            ! | ! if let _ = split_last(&()) => {}
            _ => {}
        }
    }
}
