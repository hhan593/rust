use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    // children 使用 Rc，因为父节点需要“拥有”子节点的所有权
    children: RefCell<Vec<Rc<Node>>>,

    // parent 使用 Weak，因为子节点不应该“拥有”父节点
    // 如果这里也用 Rc，就会形成：父->子(Rc) 且 子->父(Rc)，导致计数永远不归零（内存泄漏）
    parent: RefCell<Weak<Node>>,
}

fn main() {
    // 1. 创建叶子节点 leaf
    // 最初它没有父节点，所以 parent 指向一个空的 Weak 引用
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    // 尝试通过 upgrade 获取父节点。因为现在是 Weak::new()，会返回 None
    // upgrade() 方法的作用是：检查父节点是否还活着，如果活着就返回 Some(Rc<T>)
    println!("leaf parent before = {:?}", leaf.parent.borrow().upgrade());

    // 2. 创建分支节点 branch
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        // branch 拥有 leaf 的克隆，此时 leaf 的强引用计数 (strong_count) 会变为 2
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 3. 建立反向链接：让 leaf 指向它的父节点 branch
    // 使用 .borrow_mut() 获取 RefCell 内部的可变借用
    // 使用 Rc::downgrade 将 branch 的 Rc 转换为 Weak，存入 leaf 的 parent 中
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // 4. 再次查看 leaf 的父节点
    // 此时 upgrade() 会成功，返回 Some(branch)
    println!("leaf parent after = {:?}", leaf.parent.borrow().upgrade());
}

// 在 Rust 的智能指针体系中，`upgrade` 是专门为 **`Weak<T>`**（弱引用）设计的一个核心方法。

// 简单来说：**`upgrade` 的作用是尝试把“弱引用”临时变回“强引用”，以便你能安全地访问里面的数据。**

// ---

// ### 1. 为什么需要 upgrade？

// `Weak<T>` 像是一个“观察者”，它不拥有数据，也不保证数据一定存在。

// * **强引用 (`Rc`)**：只要有一个强引用在，数据就不会被销毁。
// * **弱引用 (`Weak`)**：它不影响计数。如果所有强引用都消失了，数据就会被清理，此时弱引用就指向了“虚无”。

// 因为数据可能已经不在了，你**不能直接**访问 `Weak` 指向的内容。你必须先通过 `upgrade` 问一句：“喂，你还在吗？”

// ---

// ### 2. upgrade 的返回值

// `upgrade` 返回的是一个 `Option<Rc<T>>`：

// * **`Some(Rc<T>)`**：数据还在！它会给你一个新的强引用，保证你在处理数据期间，数据不会被意外销毁。
// * **`None`**：数据已经被清理了。

// ---

// ### 3. 代码演示

// ```rust
// use std::rc::Rc;

// fn main() {
//     let strong = Rc::new(5);

// 从强引用创建一个弱引用
//     let weak = Rc::downgrade(&strong);

// 情况 A：强引用还存在
//     match weak.upgrade() {
//         Some(rc) => println!("数据还在，值是: {}", rc),
//         None => println!("数据已经没了"),
//     }

// 手动释放唯一的强引用
//     drop(strong);

// 情况 B：强引用已经没了
//     match weak.upgrade() {
//         Some(rc) => println!("数据还在: {}", rc),
//         None => println!("数据已经没了！upgrade 返回了 None"),
//     }
// }

// ```

// ---

// ### 4. 为什么要这么设计？（核心逻辑）

// 如果 `Weak` 可以直接访问数据，可能会发生**内存安全问题**：

// 1. 假设你正在读取 `Weak` 里的数据。
// 2. 另一个线程或另一段逻辑突然释放了最后一个 `Rc`。
// 3. 数据被销毁，你手里的指针变成了“悬垂指针”，导致程序崩溃。

// **`upgrade` 的精妙之处**在于：
// 当它返回 `Some(Rc<T>)` 时，它会**临时把强引用计数加 1**。这意味着只要你手里还拿着 `upgrade` 出来的这个 `Rc`，哪怕原来的那个 `Rc` 被 `drop` 了，内存依然是安全的。

// ---

// ### 总结

// * **`downgrade`**: `Rc` -> `Weak` (为了打破循环引用，不增加计数)。
// * **`upgrade`**: `Weak` -> `Option<Rc>` (为了安全地访问可能已经消失的数据)。
