use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    // 为了支持循环引用，我们需要 Rc 来共享所有权，RefCell 来提供修改能力
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // 返回 Option 包含内部的 RefCell，以便后续修改
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // a 指向 Nil
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    println!("a next item = {:?}", a.tail());

    // b 指向 a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b created = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail());

    // --- 制造循环引用 ---
    // 如果你想让 a 指回 b，可以这样做：
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count after backlink = {}", Rc::strong_count(&a)); // 2
    println!("b rc count after backlink = {}", Rc::strong_count(&b)); // 2

    // ⚠️ 注意：此时打印 a 或 b 会导致栈溢出，因为它们在互相无限引用对方
    // println!("{:?}", a.tail());
}