//#[warn(long_running_const_eval)]
fn main() {
    for _ in 0..0 {
        [(); 1];
    }

    for _ in 0..0 {
        [(); loop {}];
    }
}
