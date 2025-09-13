#![feature(trait_alias)]

struct A {}
impl A {
  fn a() -> i32 {1}
}
const a = A::a();

// enum Hoge {
//   Fuga,
//   Nyan,
// }
// const B = Hoge::Fuga;
// use std::mem::{MaybeUninit};
// const BIG_CHAIN = MaybeUninit::<&i32>::uninit();
// const BIG_CHAIN: i32 = MaybeUninit::<&i32>::uninit();
// const A = 10;
// pub trait PosSend = Send;
// pub trait PosSend2 = Send;
// pub trait PosSend3 = PosSend2;
pub trait NeverSend = !Send;

fn main() {}


