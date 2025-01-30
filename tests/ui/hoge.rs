fn main() {
    // let Some(y) = &mut Some(0);
    if let &Some(Some(x)) = &Some(&mut Some(0)) {}
}
