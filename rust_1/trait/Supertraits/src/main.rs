// === 知识点总结 ===
// 1. 超特征 (Supertraits)：`trait SubTrait: SuperTrait` 表示依赖关系。要实现子特征，必须先实现超特征。这提供了类似“接口继承”的功能。
// 2. 组合优于继承 (Composition over Inheritance)：Rust 没有类的继承。结构体不能继承结构体，只能通过实现多个 Trait 来组合不同的能力。
// 3. 内部可变性与方法委托 (Delegation)：通过封装标准库的集合（如 VecDeque），将我们自定义特征的方法直接委托给底层数据结构的内置方法来实现。
// 4. Option 枚举 (Option Enum)：`Option<T>` 用于表示一个值可能存在（`Some(T)`）或不存在（`None`），是 Rust 处理空值的安全机制。

use std::collections::VecDeque;
// 知识点：模块引入。解释：引入标准库提供的高效双端队列底层数据结构。

//继承思想 // 知识点：代码注释。解释：标明当前代码段的主题是演示特征的“继承”（超特征）。
trait Queue {
    // 知识点：定义特征。解释：定义基础的队列能力接口（FIFO，先进先出）。
    fn len(&self) -> usize; // 知识点：方法签名。解释：获取队列长度，返回 usize。
    fn is_empty(&self) -> bool; // 知识点：方法签名。解释：判断队列是否为空，返回 bool。
    fn push_back(&mut self, x: i32); // 知识点：方法签名。解释：在队尾追加元素，需要可变借用 `&mut self`。
    fn pop_front(&mut self) -> Option<i32>; // 知识点：方法签名。解释：从队头弹出元素，由于队列可能为空，所以返回 Option<i32>。
} // 知识点：作用域结束。

trait Deque: Queue {
    // 知识点：超特征 (Supertrait)。解释：定义双端队列特征 Deque，它“继承”自 Queue。意味着任何实现 Deque 的类型，必须同时也实现 Queue。
    fn push_front(&mut self, x: i32); // 知识点：方法签名。解释：在队头插入元素。
    fn pop_back(&mut self) -> Option<i32>; // 知识点：方法签名。解释：从队尾弹出元素。
} // 知识点：作用域结束。

struct MyDeque {
    // 知识点：结构体定义。解释：自定义一个结构体，作为我们实现特征的载体。
    data: VecDeque<i32>, // 知识点：组合、结构体字段。解释：内部封装一个标准库的 VecDeque 来实际存储数据。
} // 知识点：作用域结束。

impl MyDeque {
    // 知识点：实现关联函数。解释：为 MyDeque 实现自身的方法。
    fn new() -> Self {
        // 知识点：关联函数 (构造器)。解释：通常叫 new，用于初始化实例。
        MyDeque {
            // 知识点：结构体实例化。
            data: VecDeque::new(), // 知识点：底层结构初始化。解释：调用底层 VecDeque 的 new 方法创建一个空队列。
        } // 知识点：隐式返回。
    } // 知识点：作用域结束。
} // 知识点：作用域结束。

// 【强制要求 1】：因为想要实现 Deque，所以必须先把超特征 Queue 实现了。
impl Queue for MyDeque {
    // 知识点：特征实现。解释：为 MyDeque 实现基础的 Queue 特征。
    fn len(&self) -> usize {
        // 知识点：方法重写。
        self.data.len() // 知识点：方法委托、隐式返回。解释：直接调用底层 data 的 len() 方法。
    } // 知识点：作用域结束。

    fn is_empty(&self) -> bool {
        // 知识点：方法重写。
        self.data.is_empty() // 知识点：方法委托、隐式返回。解释：直接调用底层 data 的 is_empty() 方法。
    } // 知识点：作用域结束。

    fn push_back(&mut self, x: i32) {
        // 知识点：方法重写、可变借用。
        self.data.push_back(x); // 知识点：方法委托、语句。解释：将元素追加到底层 data 的队尾。
    } // 知识点：作用域结束。

    fn pop_front(&mut self) -> Option<i32> {
        // 知识点：方法重写。
        self.data.pop_front() // 知识点：方法委托、隐式返回。解释：从底层 data 队头弹出元素。
    } // 知识点：作用域结束。
} // 知识点：作用域结束。

// 【强制要求 2】：Queue 实现完毕后，编译器才允许我们实现 Deque。
impl Deque for MyDeque {
    // 知识点：特征实现。解释：为 MyDeque 实现高级的 Deque 特征。
    fn push_front(&mut self, x: i32) {
        // 知识点：方法重写。
        self.data.push_front(x); // 知识点：方法委托、语句。解释：在底层 data 的队头插入元素。
    } // 知识点：作用域结束。

    fn pop_back(&mut self) -> Option<i32> {
        // 知识点：方法重写。
        self.data.pop_back() // 知识点：方法委托、隐式返回。解释：从底层 data 队尾弹出元素。
    } // 知识点：作用域结束。
} // 知识点：作用域结束。

// === 知识点总结 ===
// 1. 静态分发的多态 (impl Trait)：作为参数时，`impl Trait` 是泛型特征约束的语法糖。它表示“我接受任何实现了该特征的类型”。编译器在编译时会为其生成具体类型的代码（零成本抽象）。
// 2. 可变借用 (&mut)：因为出队（pop）操作会实质性地修改队列内部的数据，所以函数必须要求传入参数的可变引用。
// 3. Option 枚举：Rust 中没有 null 概念。可能不存在的值使用 `Option<T>` 表示，包含 `Some(T)`（有值）和 `None`（无值）。
// 4. 条件循环模式匹配 (while let)：专门用于处理 `Option` 或 `Result` 的循环。只要等号右边的返回值能匹配左边的模式（如 Some），就会解包并执行循环体；一旦匹配失败（如遇到 None），循环自动终止。
//
fn consume_queue(q: &mut impl Queue) {
    // 知识点：impl Trait 与可变借用。解释：声明函数接收任何实现了 Queue 特征类型的可变引用。调用者不需要知道具体的底层类型是什么。
    while let Some(val) = q.pop_front() {
        // 知识点：while let 模式匹配。解释：持续调用 pop_front()。如果拿到 Some，就把里面的值绑定给 val 并进入循环；如果拿到 None（队列空了），循环立刻安全退出。
        println!("Consumed from queue: {}", val); // 知识点：标准输出。解释：打印被成功取出的值。
    } // 知识点：作用域结束。解释：当前循环步结束，回到 while let 再次调用 pop_front()。
} // 知识点：作用域结束。解释：函数执行完毕。

fn main() {
    // 知识点：程序入口。
    let mut my_q = MyDeque::new(); // 知识点：变量绑定、可变性。解释：声明一个可变的 MyDeque 实例。

    // 使用 Deque 的方法 (双端操作)
    my_q.push_front(1); // 知识点：方法调用。解释：队头插入 1。当前状态: [1]
    my_q.push_back(2); // 知识点：方法调用。解释：队尾插入 2。当前状态: [1, 2]
    my_q.push_front(0); // 知识点：方法调用。解释：队头插入 0。当前状态: [0, 1, 2]

    println!("Deque length: {}", my_q.len()); // 知识点：方法调用、格式化输出。解释：这里调用的是来自 Queue 特征的 len 方法。

    // 我们可以安全地把实现了 Deque 的实例，传给只要求 Queue 特征的函数
    consume_queue(&mut my_q); // 知识点：多态函数调用。解释：传入 my_q 的可变借用。因为它实现了 Queue（由于 Deque 的强制约定），所以完美匹配。
} // 知识点：作用域结束。解释：main 结束。
