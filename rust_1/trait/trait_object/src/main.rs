use std::fmt::Display;
pub trait Summary {
    fn summarize(&self) -> String;
}
// 多重 Trait 约束
pub fn notify(item: &(impl Summary + Display)) {
    println!("多种trait约束")
}

fn main() {}
