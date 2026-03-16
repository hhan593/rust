// `Deref` trait 允许智能指针重载解引用运算符 `*`，使其行为与普通引用一致

//先调用 `deref()` 方法获取内部值的常规引用，再通过 `*` 解引用取得值,这样时为了保护所有权
use std::cell::RefCell;
use std::ops::Deref; // 必须引入这个 Trait 才能实现解引用功能
use std::rc::Rc;
// 定义一个元组结构体 MyBox，它包裹了一个泛型 T
struct MyBox<T>(T);

impl<T> MyBox<T> {
    // 模拟 Box::new 的构造函数
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 为 MyBox 实现 Deref Trait
impl<T> Deref for MyBox<T> {
    // 定义关联类型 Target，表示解引用后得到的目标类型
    type Target = T;

    // 实现 deref 方法
    // 它借用 self 并返回指向内部数据的引用 &T
    fn deref(&self) -> &Self::Target {
        &self.0 // 返回元组结构体的第一个元素（即包裹的数据）的引用
    }
}

//### 2.3 Deref Coercion（自动解引用转换）

fn hello(name: &str) -> &str {
    println!("Hello,{}!", name);
    name //没有这一行的话会报错，函数没有呢返回值zhi符串，所以会报错
}
// fn main() {
// --- 场景 1: 标准引用 ---
// let x = 5;
// let y = &x;        // y 是一个指向 x 的引用
// assert_eq!(5, x);
// assert_eq!(5, *y);  // 手动解引用 y 以获得其指向的值

// --- 场景 2: 自定义智能指针 ---
//     let x = 5;
//     let y = MyBox::new(x); // 将 x 的所有权移入 MyBox 实例

// 这里是重点：
//      因为 MyBox 实现了 Deref，当我们使用 *y 时，底层发生了以下转换：
//      1. 编译器发现 * 运算符作用于 MyBox。
// 2. 调用 y.deref()，返回 &x（即内部值的引用）。
// 3. 再次解引用这个返回的引用，得到最终的 5。
//     assert_eq!(5, *y);

//### 2.3 Deref Coercion（自动解引用转换）

//     let m = MyBox::new(String::from("Rust"));
// MyBox<String> -> &String -> &str（自动解引用链）
//     hello(&m);

//     **三条转换规则：**

// 1. `&T` → `&U`，当 `T: Deref<Target=U>`（不可变到不可变）
// 2. `&mut T` → `&mut U`，当 `T: DerefMut<Target=U>`（可变到可变）
// 3. `&mut T` → `&U`，当 `T: Deref<Target=U>`（可变到不可变）
//     let mut str1 = String::from("hello");

// --- 场景：试图获取 hello 的返回值 ---
// hello(&str1) 发生了 Deref Coercion: &String -> &str
// hello 返回的是 &str，它其实是 str1 的一个不可变借用
//     let str2 = hello(&str1);

// ✅ 这一行本身没问题，因为 str2 只是读取数据
//     println!("{}", str2);

// ❌ 为什么此处如果尝试修改 str1 会报错？
// str1.push_str(" world"); // 如果解开这行注释，下一行的 println! 就会报错
// 因为 str2 依然持有对 str1 的【不可变借用】，根据规则：
// 在存在不可变借用时，不能存在可变借用（修改数据）
//     println!("{}", str2)

// 反向不可行：不可变引用永远不会自动转换为可变引用。

// > Deref coercion 在编译期解析，**零运行时开销**。
// }
// --- 重点解释：为什么反向转换（不可变 -> 可变）不行？ ---

fn manual_example() {
    let x = 5;
    let y = &x; // y 是 &i32（不可变引用）

    // 假设 Rust 允许 &T -> &mut U 的转换：
    // let z: &mut i32 = y; // ❌ 编译错误！
    // *z = 10;            // 如果允许，这里就篡改了原本声明为不可变的数据 x

    // 核心原因：
    // 1. 内存安全：不可变引用保证了数据在引用的生命周期内不会被改变。
    // 2. 避免竞态：如果多个不可变引用中隐藏着一个可变引用，会引发不可预知的并发问题。
}

//## 三、Drop Trait — 自动资源清理

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// 提前释放 std::mem::drop
// Rust 不允许直接调用 `drop()` 方法（会导致 double free）。如果需要提前释放，使用标准库的 `std::mem::drop`：

// fn main() {
// 1. 压栈：c 第一个进入作用域
//     let c = CustomSmartPointer {
//         data: String::from("hello"),
//     };

// 2. 压栈：d 第二个进入作用域
//     let d = CustomSmartPointer {
//         data: String::from("world"),
//     };

//     println!("CustomSmartPointers created.");

// --- 作用域结束点 ---
// 3. 出栈（逆序）：
//    由于 d 是最后定义的，它首先被弹出并调用 drop()。
//    打印: "Dropping CustomSmartPointer with data `world`!"

// 4. 继续出栈：
//    接着轮到 c 被弹出并调用 drop()。
//    打印: "Dropping CustomSmartPointer with data `hello`!"

//     /* *********************## 3.2 提前释放：`std::mem::drop`*************************/
//Rust 不允许直接调用 `drop()` 方法（会导致 double free）。如果需要提前释放，使用标准库的 `std::mem::drop`：

//   通常情况下，变量 c 的生命周期会持续到 main 函数的最后一刻

//     let e = CustomSmartPointer {
//         data: String::from("hello"),
//     };
//     println!("CustomSmartPointer created.");
//     drop(e); // 提前释放
//     println!("CustomSmartPointer dropped before end of main.");

// --- 正常释放点 ---
// 如果没有手动 drop，c 会在这里（函数结束时）被释放
// 当你调用 drop(c) 时，你是在函数中途强制执行了原本应该在结尾才发生的清理动作、

// 它的工作原理如下：

// 夺取所有权：当你调用 drop(c) 时，变量 c 的所有权被**移动（Move）**到了 drop 函数的参数 _x 中。

// 函数结束：一旦 drop 函数执行完毕（即到达它的 }），参数 _x 离开了它的作用域。

// 自动清理：因为 _x 现在拥有该数据的所有权，Rust 的析构机制被触发，执行 CustomSmartPointer 的 drop 方法。

// 结论：drop(c) 并不是真的有一个“销毁开关”，它只是通过提前夺取所有权，让变量在 drop 函数内部自然死亡。

// drop不可以手动调用
// }

// 四、`Box<T>` — 堆分配与独占所有
// `Box<T>` 是最简单的智能指针，将数据分配在**堆**上，栈上只保存指向堆数据的指针。类似 C++ 的 `unique_ptr`。

// 五、`Rc<T>` — 引用计数与共享所有权
//`Rc<T>`（Reference Counting）允许一个值拥有**多个所有者**。内部维护一个引用计数，当最后一个 `Rc` 被销毁时，数据才会被释放。

// 定义一个递归列表（Cons List）
// 使用 Rc<List> 而不是 Box<List>，因为我们需要多个节点指向同一个后续节点
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

// fn main() {
// 1. 创建列表 a，它是 5 -> 10 -> Nil
// Rc::new 会在堆上分配空间，并初始化引用计数为 1
// let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
// println!("count after creating a = {}", Rc::strong_count(&a)); // 打印：1

// 2. 创建列表 b，它的第一个元素是 3，后面指向 a
// Rc::clone(&a) 不会深拷贝数据，它只是增加 a 的引用计数（强引用计数）
// let b = Cons(3, Rc::clone(&a));
// println!("count after creating b = {}", Rc::strong_count(&a)); // 打印：2

// {
// 3. 进入内部作用域，创建列表 c，指向 a
// 此时 a 的引用计数再次增加
// let c = Cons(4, Rc::clone(&a));
// println!("count after creating c = {}", Rc::strong_count(&a)); // 打印：3

// 当内部作用域结束时，c 被销毁。
// 因为 c 内部持有一个 Rc::clone(&a)，c 的销毁会触发对应 Rc 的 drop。
// 这会自动将 a 的引用计数减 1。
// }

// 4. 此时 c 已不在，引用计数恢复为 2（只有 a 本身和 b 在引用它）
// println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 打印：2

// 当 main 函数结束，b 和 a 依次销毁，计数最终归零，堆内存被释放。
// }
// 这段代码展示了 Rust 中的 **`Rc<T>`（Reference Counted，引用计数）** 智能指针的使用场景。

// 在 Rust 中，通常一个值只能有一个所有者。但当你需要多个变量（比如这里的 `b` 和 `c`）同时共同拥有并指向同一块内存数据（比如 `a`）时，`Rc<T>` 就派上用场了。

// ### 代码详细注释

// ```rust

// 定义一个递归列表（Cons List）
// 使用 Rc<List> 而不是 Box<List>，因为我们需要多个节点指向同一个后续节点
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main() {
// 1. 创建列表 a，它是 5 -> 10 -> Nil
// Rc::new 会在堆上分配空间，并初始化引用计数为 1
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a)); // 打印：1

// 2. 创建列表 b，它的第一个元素是 3，后面指向 a
// Rc::clone(&a) 不会深拷贝数据，它只是增加 a 的引用计数（强引用计数）
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a)); // 打印：2

//     {
// 3. 进入内部作用域，创建列表 c，指向 a
// 此时 a 的引用计数再次增加
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a)); // 打印：3

// 当内部作用域结束时，c 被销毁。
// 因为 c 内部持有一个 Rc::clone(&a)，c 的销毁会触发对应 Rc 的 drop。
// 这会自动将 a 的引用计数减 1。
//     }

// 4. 此时 c 已不在，引用计数恢复为 2（只有 a 本身和 b 在引用它）
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 打印：2

// 当 main 函数结束，b 和 a 依次销毁，计数最终归零，堆内存被释放。
// }

// ```

// ---

// ### 💡 核心知识点补充

// #### 1. 为什么用 `Rc::clone(&a)` 而不是 `a.clone()`？

// 虽然两者效果一样，但 Rust 习惯上使用 `Rc::clone`。

// * **性能**：`Rc::clone` 只是增加计数，速度极快。普通的 `.clone()` 在很多类型上意味着深拷贝数据。
// * **语义**：一眼就能看出这里只是在增加引用计数，而不是在复制整棵“列表树”。

// #### 2. `Rc<T>` 的局限性

// * **只读性**：`Rc<T>` 允许数据有多个所有者，但它是**不可变引用**。如果你想修改 `Rc` 里的数据，你需要配合 `RefCell<T>`（即 `Rc<RefCell<T>>`）。
// * **单线程**：`Rc` 不是线程安全的。如果你要在多线程中使用，必须换成 `Arc<T>` (Atomic Reference Counted)。

// #### 3. 内存结构示意

// 这段代码在执行到 `c` 还在作用域时，内存中的拓扑结构如下：

// * **b** -> `[3]`
//                 **↘**
// * **a** ----------> **`[5]`** -> `[10]` -> `Nil`
// * **c** -> `[4]` /
//                 **↗**

// > 这就是典型的“多对一”关系，`a` 成了公共的末端，被三方共享。

// **你想看看如果想要修改 `a` 中的数据，应该如何配合 `RefCell` 来操作吗？**

fn main() {
    // 1. 虽然 data 本身是不可变的 (let 没有 mut)
    let data = RefCell::new(5);

    {
        // 2. 获取内部数据的可变借用
        let mut a = data.borrow_mut(); // 可变借用
        *a += 10;
    } // 3. 离开作用域，数据被释放， 可变借用在这里被 drop，计数器重置

    // 3. 获取不可变借用并读取，已经被改变了

    println!("Value: {:?}", data.borrow()); // 输出: Value: 15
}

//`RefCell` 会在运行时维护一个计数器。如果你尝试在同一作用域内同时进行两个可变借用，程序会直接 **panic**，而不是产生数据竞争。
// let data = RefCell::new(5);

// let m1 = data.borrow_mut();
// let m2 = data.borrow_mut(); // ❌ 运行到这里程序会直接崩溃！
// 报错：already borrowed: BorrowMutError
