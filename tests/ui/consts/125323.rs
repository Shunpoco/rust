//@ run-pass
fn should_not_be_dead() {}

fn main() {
    for _ in 0..0 {
        [(); loop {}];
    }

    should_not_be_dead();
}
