// skip-filecheck
fn split_last(_: &()) -> Option<(&i32, &i32)> {
    None
}

fn hoge(x: i32) -> i32 {
    let y = 5;
    match x {
        (10 | !) if y == 5 => return 10,
        _ => return x * 10,
    }
}


fn assign_twice() {
    hoge(10);
    loop {
        match () {
            
            (! | !) if let _ = split_last(&()) => {}
            _ => {}
        }
    }
}
