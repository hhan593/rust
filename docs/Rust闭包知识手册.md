# Rust 闭包（Closures）完全指南

## 目录

1. [闭包的定义和语法](#1-闭包的定义和语法)
2. [闭包捕获环境变量的三种方式](#2-闭包捕获环境变量的三种方式)
3. [move 关键字的使用](#3-move-关键字的使用)
4. [闭包作为函数参数和返回值](#4-闭包作为函数参数和返回值)
5. [闭包的类型推断](#5-闭包的类型推断)
6. [闭包与函数指针的区别](#6-闭包与函数指针的区别)
7. [实际使用场景和示例代码](#7-实际使用场景和示例代码)

---

## 1. 闭包的定义和语法

### 什么是闭包？

闭包是一个能够**捕获周围环境变量**的**匿名函数**。它是 Rust 中强大的函数式编程特性。

### 闭包的多种写法

```rust
fn main() {
   fn main() {
    // 1. 最简洁的单行闭包
    // |x| 是参数列表，x * x 是表达式也是返回值。
    // Rust 会根据第一次调用（square(5)）自动推导出 x 的类型为 i32。
    let square = |x| x * x;
    println!("{}", square(5));  // 输出: 25

    // 2. 带类型注解的闭包
    // 语法与普通函数类似，但在不需要推导或为了提高代码可读性时可以显式标注类型。
    let add = |x: i32, y: i32| -> i32 { x + y };
    println!("{}", add(3, 4));  // 输出: 7

    // 3. 多行闭包（使用花括号）
    // 当闭包逻辑复杂时，使用 {} 包裹，最后一行（不带分号）作为返回值。
    let greet = |name: &str| {
        let greeting = format!("你好, {}", name);
        println!("{}", greeting);
        greeting // 返回生成的字符串
    };
    greet("小明");

    // 4. 捕获环境变量的闭包
    // 这是闭包与普通函数最大的区别：它可以直接访问定义在它之外的作用域中的变量 `num`。
    let num = 5;
    let add_num = |x| x + num;
    println!("{}", add_num(3));  // 输出: 8

    // 5. 没有参数的闭包
    // 使用空的竖线 || 表示不接收任何输入。
    let get_value = || 42;
    println!("{}", get_value());  // 输出: 42

    // 6. 捕获多个变量的闭包
    // 闭包可以同时捕获作用域内的多个变量（x 和 y）。
    // 默认情况下，闭包会以“不可变借用”的方式捕获这些变量。
    let x = 10;
    let y = 20;
    let combine = || x + y;
    println!("{}", combine());  // 输出: 30
}
}
```

### 闭包 vs 普通函数语法对比

```rust
// 普通函数：必须显式声明所有类型
fn add_one_fn(x: i32) -> i32 { x + 1 }

// 闭包的等价写法（从完整到简化）
let add_one_v1 = |x: i32| -> i32 { x + 1 };  // 1. 完整标注类型和花括号
let add_one_v2 = |x: i32|          x + 1  ;   // 2. 省略返回类型和花括号（仅限单行表达式）
let add_one_v3 = |x|               x + 1  ;   // 3. 省略参数类型（由编译器根据上下文推断）
```

---

## 2. 闭包捕获环境变量的三种方式

Rust 中闭包捕获变量的方式对应三个 trait：`Fn`、`FnMut`、`FnOnce`。编译器根据闭包如何使用环境变量来自动推断。

### 2.1 Fn — 不可变借用

闭包只读取环境变量，不做任何修改。可以调用多次。

```rust
fn main() {
    let config = vec![1, 2, 3];

    // 闭包以“不可变借用” (&T) 的方式捕获 config
    let print_config = || {
        println!("配置: {:?}", config); // 只读访问
        println!("配置长度: {}", config.len());
    };

    print_config(); // 第一次调用
    print_config(); // 可以多次调用，因为只是读取

    // 原变量仍然可用，因为闭包只是借用了它
    println!("config 仍然存在: {:?}", config);
}
```

### 2.2 FnMut — 可变借用

闭包需要修改环境变量。可以调用多次，但在闭包活动期间，环境变量被可变借用，此时是不可以使用number，因为被借用了。

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    // 闭包需要修改 numbers，因此它以“可变借用” (&mut T) 方式捕获
    // 注意：闭包变量本身必须声明为 mut，因为调用它会改变闭包内部的状态
    let mut add_to_list = |x| {
        numbers.push(x); // 修改了外部变量
        println!("添加 {}，列表现在有 {} 个元素", x, numbers.len());
    };

    add_to_list(4);
    add_to_list(5); // 可以多次调用

    // 在闭包的生命周期结束（或手动 drop）之前，numbers 一直被闭包占用
    drop(add_to_list);
    println!("最终列表: {:?}", numbers); // 此时可以再次安全访问 numbers
}
```

### 2.3 FnOnce — 所有权转移

闭包捕获变量的所有权，只能调用一次。

```rust
fn main() {
    let s = String::from("hello");

    // 这个闭包捕获了 s 的所有权（因为调用了 drop(s)）
    let consume = || {
        let upper = s.to_uppercase();
        println!("消费: {}", upper);
        drop(s);  // 显式消耗变量，导致 s 离开作用域
    };

    consume(); // 执行闭包
    // consume();  // 错误！s 已被消费，该闭包只能调用一次 (FnOnce)
    // println!("{}", s);  // 错误！s 的所有权已转移进闭包并被销毁了
}
```

### 三种 Trait 对比表

| Trait    | 捕获方式   | 可调用次数 | 使用场景         |
| -------- | ---------- | ---------- | ---------------- |
| `Fn`     | 不可变借用 | 无限       | 只读取环境变量   |
| `FnMut`  | 可变借用   | 无限       | 修改环境变量     |
| `FnOnce` | 所有权转移 | 一次       | 消耗或转移所有权 |

### Trait 之间的继承关系

```text
FnOnce  ←  FnMut  ←  Fn
（最宽泛）          （最严格）
```

- 所有闭包都实现了 `FnOnce`
- 实现了 `Fn` 的闭包也自动实现了 `FnMut` 和 `FnOnce`
- 实现了 `FnMut` 的闭包也自动实现了 `FnOnce`

---

## 3. move 关键字的使用

`move` 关键字强制闭包**获取其捕获变量的所有权**，而不是借用。

### 基本用法

```rust
fn main() {
    let s = String::from("hello");

    // 没有 move：编译器推断借用
    let closure1 = || println!("{}", s);  // 不可变借用
    closure1();
    println!("{}", s);  // 仍然可以使用 s

    let s2 = String::from("world");

    // 使用 move：强制转移所有权
    let closure2 = move || println!("{}", s2);
    closure2();
    // println!("{}", s2);  // 错误！所有权已被转移
}
```

### 最常见场景：多线程

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // 必须使用 move，确保变量所有权转移到新线程
    let handle = thread::spawn(move || {
        println!("线程中的向量: {:?}", v);
    });

    // println!("{:?}", v);  // 错误！v 已被移入线程
    handle.join().unwrap();
}
```

### move 对 Copy 类型的影响

```rust
fn main() {
    let x = 42;  // i32 实现了 Copy

    // move 对 Copy 类型会复制值，原变量仍可用
    let closure = move || println!("{}", x);
    closure();
    println!("{}", x);  // 仍然可以使用 x（因为 i32 是 Copy 类型）
}
```

---

## 4. 闭包作为函数参数和返回值

### 4.1 闭包作为函数参数

```rust
// 使用泛型 + trait bound
// -----------------------------------------------------------------------------
// 1. Fn Trait
// -----------------------------------------------------------------------------
// F: Fn(i32) -> i32
// 含义：F 是一个闭包或函数，它接受一个 i32 参数，返回一个 i32。
// 特点：
//   - 通过不可变引用 (&self) 调用。
//   - 闭包内部不能修改其捕获的环境变量。
//   - 可以无限次调用。
// 适用场景：只读访问外部变量的闭包。
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

// -----------------------------------------------------------------------------
// 2. FnMut Trait (使用 where 子句写法)
// -----------------------------------------------------------------------------
// F: FnMut(i32) -> i32
// 含义：F 是一个闭包，接受 i32 返回 i32。
// 特点：
//   - 通过可变引用 (&mut self) 调用。
//   - 闭包内部可以修改其捕获的环境变量。
//   - 可以多次调用。
// 注意：参数 f 必须声明为 mut (mut f: F)，因为调用 FnMut 需要可变借用。
// 适用场景：需要累加、计数或修改外部状态的闭包。
fn apply_mut<F>(mut f: F, x: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(x)
}

// -----------------------------------------------------------------------------
// 3. FnOnce Trait
// -----------------------------------------------------------------------------
// F: FnOnce(i32) -> i32
// 含义：F 是一个闭包，接受 i32 返回 i32。
// 特点：
//   - 通过值 (self) 调用，意味着调用后闭包本身会被“消耗”（move）。
//   - 闭包内部可以移动（consume）其捕获的变量。
//   - 只能调用一次（因为调用后它就不存在了）。
// 关系：Fn 和 FnMut 都自动实现了 FnOnce（因为如果能重复调用，自然也能调用一次）。
// 适用场景：一次性操作，或者需要拥有捕获变量所有权的闭包（例如线程启动、资源清理）。
fn apply_once<F: FnOnce(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn main() {
    // 定义一个简单的闭包：将输入乘以 2
    // 这个闭包不捕获任何外部变量，因此它同时实现了 Fn, FnMut, 和 FnOnce。
    let double = |x| x * 2;

    // 调用 apply (需要 Fn)
    // double 可以通过不可变引用调用，完全符合要求。
    println!("{}", apply(double, 5));  // 输出: 10

    // 下面的代码演示了不同 trait 的实际区别（仅供理解，原代码未执行）：

    /*
    // 场景 A: 修改外部变量 (需要 FnMut)
    let mut sum = 0;
    let mut add_to_sum = |x| { sum += x; };
    // apply(add_to_sum, 5);  // 错误！apply 需要 Fn，但 add_to_sum 修改了 sum，只实现了 FnMut
    apply_mut(add_to_sum, 5); // 正确！apply_mut 需要 FnMut

    // 场景 B: 移动外部变量 (需要 FnOnce)
    let s = String::from("hello");
    let consume = |x| {
        drop(s); // 移动并丢弃 s
        x
    };
    // apply(consume, 5);     // 错误！consume 移动了 s，只实现了 FnOnce
    // apply_mut(consume, 5); // 错误！同上
    apply_once(consume, 5);   // 正确！apply_once 需要 FnOnce，调用后 consume 被销毁
    */
}
// 1. 接受 Fn 的函数：闭包不能修改捕获的变量
fn run_fn<F>(f: F, val: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(val)
}

// 2. 接受 FnMut 的函数：闭包可以修改捕获的变量
fn run_fn_mut<F>(mut f: F, val: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(val)
}

// 3. 接受 FnOnce 的函数：闭包可以消耗（移动）捕获的变量
fn run_fn_once<F>(f: F, val: i32) -> i32
where
    F: FnOnce(i32) -> i32,
{
    f(val)
}

fn main() {
    println!("=== 场景 1: Fn (只读) ===");
    let multiplier = 10;
    // 这个闭包只读取了 multiplier，没有修改它
    let multiply = |x| x * multiplier;

    // 可以多次调用，因为它不改变状态
    println!("结果 1: {}", run_fn(multiply, 5)); // 50
    println!("结果 2: {}", run_fn(multiply, 6)); // 60
    // multiplier 依然可用
    println!("外部变量 multiplier: {}", multiplier);


    println!("\n=== 场景 2: FnMut (修改状态) ===");
    let mut counter = 0;
    // 这个闭包修改了外部变量 counter (使用了 +=)
    // 因此它只实现了 FnMut 和 FnOnce，**没有**实现 Fn
    let mut add_and_count = |x| {
        counter += 1; // 修改捕获的环境
        x + counter
    };

    // ❌ 下面这行会报错，因为 run_fn 需要 Fn，但 add_and_count 是 FnMut
    // println!("{}", run_fn(add_and_count, 5));

    // ✅ 必须使用 run_fn_mut
    println!("结果 1: {}", run_fn_mut(&mut add_and_count, 5)); // counter 变为 1, 结果 6
    println!("结果 2: {}", run_fn_mut(&mut add_and_count, 5)); // counter 变为 2, 结果 7

    // 注意：counter 的值已经被闭包修改了
    println!("外部变量 counter: {}", counter); // 输出 2


    println!("\n=== 场景 3: FnOnce (消耗所有权) ===");
    let greeting = String::from("Hello");
    // 这个闭包移动（move）了 greeting 的所有权
    // 因此它**只**实现了 FnOnce，不能再次调用
    let consume_string = |x| {
        // greeting 被移动到闭包内部，并在打印后被丢弃
        println!("闭包内使用了: {}", greeting);
        x
    };

    // ❌ 下面两行都会报错：
    // 1. run_fn 需要 Fn (不行，因为 greeting 被移动了)
    // 2. run_fn_mut 需要 FnMut (不行，同上)
    // println!("{}", run_fn(consume_string, 5));
    // println!("{}", run_fn_mut(consume_string, 5));

    // ✅ 必须使用 run_fn_once
    println!("结果: {}", run_fn_once(consume_string, 5));

    // ❌ 下面这行会报错，因为 greeting 已经在闭包调用时被移走了
    // println!("{}", greeting);

    // ❌ 下面这行也会报错，因为 consume_string 调用一次后就失效了
    // println!("{}", run_fn_once(consume_string, 5));
}
```

### 4.2 闭包作为返回值

```rust
// 方式1：使用 impl Fn（静态分发，推荐）
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

// 方式2：使用 Box<dyn Fn>（动态分发，可返回不同类型的闭包）
fn create_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        "add" => Box::new(|a, b| a + b),
        "sub" => Box::new(|a, b| a - b),
        "mul" => Box::new(|a, b| a * b),
        _     => Box::new(|a, b| a / b),
    }
}

fn main() {
    let triple = create_multiplier(3);
    println!("{}", triple(5));  // 15

    let add = create_operation("add");
    println!("{}", add(10, 20));  // 30
}
```

---

## 5. 闭包的类型推断

### 推断规则

```rust
fn main() {
    // 编译器推断参数类型为 i32
    let add = |x, y| x + y;
    let result = add(3, 4);  // 推断为 i32

    // 一旦确定了类型，就不能改变
    // let result2 = add(3.0, 4.0);  // 错误！已推断为 i32
}
```

### 每个闭包都有独特的类型

```rust
fn main() {
    // ------------------------------------------------------------------
    // 1. 演示问题：即使代码一模一样，闭包类型也是唯一的
    // ------------------------------------------------------------------

    let closure_a = |x: i32| x + 1;
    let closure_b = |x: i32| x + 1;

    // 在 Rust 中，每个闭包都有一个编译器生成的唯一匿名类型。
    // 比如 closure_a 的类型可能是 __ClosureTypeA，closure_b 是 __ClosureTypeB。
    // 它们互不兼容。

    // ❌ 下面这段代码会报错：
    /*
    let mut f = closure_a;
    f = closure_b; // 错误！expected closure type A, found closure type B
    */

    // 甚至不能直接放入同一个 Vec，因为 Vec<T> 要求所有元素类型 T 必须完全一致。
    // let tasks = vec![closure_a, closure_b]; // ❌ 报错：mismatched types

    // ------------------------------------------------------------------
    // 2. 解决方案：使用 Box<dyn Fn> 进行“类型擦除”
    // ------------------------------------------------------------------

    // 我们把具体的闭包放入 Box 中，并转换为 trait object (dyn Fn)。
    // 此时，它们的类型都变成了统一的：Box<dyn Fn(i32) -> i32>
    let tasks: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(closure_a),
        Box::new(closure_b),
        // 我们还可以混入其他逻辑完全不同的闭包，只要签名匹配
        Box::new(|x| x * 10),
        Box::new(|x| {
            println!("正在处理: {}", x);
            x - 5
        }),
    ];

    // ------------------------------------------------------------------
    // 3. 统一调用
    // ------------------------------------------------------------------
    println!("开始执行任务队列...");

    for (index, task) in tasks.iter().enumerate() {
        // task 的类型是 &Box<dyn Fn(...)>
        // 我们可以直接像调用函数一样调用它：task(input)
        let result = task(10);
        println!("任务 #{} 的结果: {}", index, result);
    }

    // ------------------------------------------------------------------
    // 4. 进阶：在结构体中存储不同类型的回调
    // ------------------------------------------------------------------
    struct Button {
        label: String,
        // 按钮点击时执行的逻辑可以是任意闭包
        on_click: Box<dyn Fn()>,
    }

    let mut count = 0;

    // 按钮 A：只是打印
    let btn_a = Button {
        label: "Print".to_string(),
        on_click: Box::new(|| println!("按钮 A 被点击了！")),
    };

    // 按钮 B：修改外部变量 (需要 FnMut，所以这里要用 Box<dyn FnMut>)
    // 注意：如果闭包内部修改了变量，Box 里的 trait 也要改成 FnMut
    let mut btn_b = Button {
        label: "Count".to_string(),
        on_click: Box::new(|| {
            // 这里的 count 需要被捕获并修改，所以这个闭包是 FnMut
            // 但我们的 Button 结构体定义的是 Fn，这里会报错！
            // 为了解决这个问题，我们通常需要在结构体里存 FnMut 或者用 RefCell
        }),
    };

    // 修正后的结构体示例 (支持修改状态)
    struct MutableButton {
        label: String,
        on_click: Box<dyn FnMut()>,
    }

    let mut counter = 0;
    let btn_count = MutableButton {
        label: "Counter".to_string(),
        on_click: Box::new(|| {
            counter += 1;
            println!("计数器现在是: {}", counter);
        }),
    };

    // 注意：调用 FnMut 需要可变借用
    let mut active_btn = btn_count;
    (active_btn.on_click)(); // 输出: 计数器现在是: 1
    (active_btn.on_click)(); // 输出: 计数器现在是: 2
}
```

---

## 6. 闭包与函数指针的区别

| 特性     | 闭包                     | 函数指针             |
| -------- | ------------------------ | -------------------- |
| 捕获变量 | 可以                     | 不能                 |
| 大小     | 可变（无捕获时为 0）     | 固定（通常 8 字节）  |
| 语法     | `\|x\| x + 1`            | `fn(i32) -> i32`     |
| 存储     | 需要泛型或 `Box<dyn Fn>` | 可直接存储           |
| 类型     | 每个闭包是独特的匿名类型 | `fn(...)` 是具体类型 |

```rust
fn double(x: i32) -> i32 { x * 2 }

fn main() {
    // 函数指针：固定大小，不能捕获变量
    let func_ptr: fn(i32) -> i32 = double;

    // 闭包：可以捕获变量
    let factor = 3;
    let closure = |x: i32| x * factor;

    // 函数指针可以直接存储在 Vec 中
    // 注意：这里必须明确告诉编译器我们要把它当成 fn 指针
    let operations: Vec<fn(i32) -> i32> = vec![
        |x: i32| x + 1, // 编译器发现目标类型是 fn，且闭包无捕获，允许转换
        |x: i32| x * 2,
    ];

    // 闭包需要 Box<dyn Fn> 才能存储在集合中
    let closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(closure),
    ];

    println!("函数指针大小: {} 字节", std::mem::size_of_val(&func_ptr));
    // 8 字节
}
```

---

## 7. 实际使用场景和示例代码

### 场景 1：懒加载（Option::unwrap_or_else）

```rust
// ============================================================================
// 场景 1：惰性求值 (Lazy Evaluation)
// ============================================================================
// 用途：避免不必要的昂贵计算。
// 原理：unwrap_or_else 接收一个闭包。只有当 Option 为 None 时，闭包才会被执行。
//       如果是 Some，闭包内的代码（如数据库查询、复杂计算）完全不会运行。

struct Inventory {
    shirts: Vec<String>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<String>) -> String {
        // 如果 user_preference 是 Some(val)，直接返回 val，闭包 || ... 被忽略。
        // 如果 user_preference 是 None，才执行 self.most_stocked()。
        // 对比：如果用 unwrap_or(self.most_stocked())，无论是否需要，most_stocked 都会先执行。
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> String {
        println!("-> [场景1] 正在执行昂贵的库存计算...");
        "Red".to_string()
    }
}

// ============================================================================
// 场景 2：迭代器链 (Iterator Chains)
// ============================================================================
// 用途：函数式数据处理管道。
// 原理：filter, map 等方法接收闭包作为参数，定义数据如何过滤和转换。
//       闭包语法简洁，非常适合这种“一次性的逻辑块”。

fn scene_iterator() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: Vec<i32> = numbers
        .iter()                 // 产生迭代器，元素类型为 &i32
        // 闭包 1: 过滤
        // 参数 x 是 &&i32 (迭代器的引用 + filter 内部的引用)，所以用 |&&x| 解构两次拿到 i32
        .filter(|&&x| x % 2 == 0)
        // 闭包 2: 转换
        // 此时元素是 &i32，用 |&x| 解构拿到 i32
        .map(|&x| x * x)
        .collect();

    println!("-> [场景2] 偶数平方结果: {:?}", result);
    // 输出: [4, 16, 36, 64, 100]
}

// ============================================================================
// 场景 3：回调和事件处理 (Callbacks & Event Handling)
// ============================================================================
// 用途：存储行为供将来调用（策略模式/观察者模式）。
// 原理：使用 Box<dyn Fn()> 将不同类型的闭包统一存储。
//       'static 约束确保闭包不持有短期借用的引用，适合长期存储。

struct Button {
    label: String,
    // 存储一个可选的、堆分配的、动态分发的函数对象
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    fn new(label: &str) -> Self {
        Button { label: label.to_string(), on_click: None }
    }

    // 泛型 F 接收任意闭包类型
    // 'static: 闭包不能借用栈上的临时变量（因为 Button 可能活得更久）
    // Fn(): 闭包无参数、无返回值、可多次调用
    fn set_on_click<F: 'static + Fn()>(&mut self, handler: F) {
        // 装箱：将具体类型的闭包转换为 trait 对象
        self.on_click = Some(Box::new(handler));
    }

    fn click(&self) {
        if let Some(handler) = &self.on_click {
            println!("-> [场景3] 按钮 '{}' 被点击，执行回调...", self.label);
            handler(); // 动态调用
        }
    }
}

// ============================================================================
// 场景 4：Result / Option 的高级用法
// ============================================================================
// 用途：优雅的错误处理和类型转换。
// 原理：利用闭包在链式调用中处理 Ok/Some 或 Err/None 的情况。

fn scene_error_handling() {
    let strings = vec!["1", "2", "abc", "4"];

    // filter_map: 结合过滤和映射
    // 闭包逻辑：尝试解析字符串。如果成功 (Ok)，返回 Some(num)；如果失败 (Err)，ok() 转为 None。
    // filter_map 会自动过滤掉 None，保留 Some 中的值。
    let numbers: Vec<i32> = strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    println!("-> [场景4] 解析成功的数字: {:?}", numbers); // [1, 2, 4]

    // map_err: 转换错误类型而不改变成功值
    // 如果解析失败，闭包将标准错误 e 转换为自定义的 String 错误信息
    let result: Result<i32, String> = "42"
        .parse::<i32>()
        .map_err(|e| format!("解析错误: {}", e));

    // 如果是 "abc"，result 将是 Err("解析错误: ...")
}

// ============================================================================
// 场景 5：多线程 (Multithreading)
// ============================================================================
// 用途：并发执行任务并共享状态。
// 原理：thread::spawn 需要 'static + Send 的闭包。
//       move 关键字强制闭包捕获变量的所有权，使其生命周期独立于主线程。
//       Arc/Mutex 用于在多个线程间安全地共享可变状态。

fn scene_multithreading() {
    use std::thread;
    use std::sync::{Arc, Mutex};

    // Arc: 原子引用计数，允许多个线程共享所有权
    // Mutex: 互斥锁，保证同一时间只有一个线程能修改内部数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        // 克隆 Arc 指针，让新线程也拥有 counter 的引用
        let counter = Arc::clone(&counter);

        // move: 将 counter (Arc 副本) 和 i 的所有权移入闭包
        // 这样闭包就可以独立于循环变量运行
        let handle = thread::spawn(move || {
            // 锁住 Mutex，获取内部数据的可变引用
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("-> [场景5] 线程 {} 完成，当前计数: {}", i, *num);
        });
        handles.push(handle);
    }

    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }

    println!("-> [场景5] 最终计数: {}", *counter.lock().unwrap());
}

// ============================================================================
// 场景 6：函数组合 (Function Composition)
// ============================================================================
// 用途：将小函数组合成大函数，构建复杂逻辑。
// 原理：compose 函数接收两个函数 f 和 g，返回一个新的闭包。
//       这个新闭包捕获了 f 和 g (通过 move)，并在被调用时依次执行它们。

// 返回类型 impl Fn(A) -> C 表示返回一个实现了 Fn trait 的匿名类型
fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    // move: 捕获 f 和 g 的所有权，使返回的闭包可以独立存在
    move |x| g(f(x))
}

fn scene_composition() {
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;

    // 生成一个新函数：先加 1，再乘 2
    let add_one_then_double = compose(add_one, double);

    let res = add_one_then_double(5); // (5 + 1) * 2 = 12
    println!("-> [场景6] 组合函数结果: {}", res);
}

// ============================================================================
// 主函数：运行所有场景
// ============================================================================
fn main() {
    // 场景 1
    let inv = Inventory { shirts: vec![] };
    // 传入 None，触发惰性计算
    let item = inv.giveaway(None);
    println!("-> [场景1] 获得的衬衫颜色: {}\n", item);

    // 场景 2
    scene_iterator();
    println!();

    // 场景 3
    let mut btn = Button::new("提交");
    btn.set_on_click(|| println!("   表单已提交！"));
    btn.click();
    println!();

    // 场景 4
    scene_error_handling();
    println!();

    // 场景 5
    scene_multithreading();
    println!();

    // 场景 6
    scene_composition();
// }
// 惰性 (unwrap_or_else): 闭包是代码块，不是立即执行的值。这让它成为控制流（如“仅在需要时计算”）的完美工具。
// 迭代器 (filter, map): 闭包提供了极简的语法来定义“对每个元素做什么”，避免了编写冗长的命名函数。
// 多态 (Box<dyn Fn>): 当需要在结构体中存储不同类型的行为，或者在运行时决定行为时，必须使用 Trait 对象。
// 错误处理 (filter_map, map_err): 闭包允许我们在链式调用中灵活地转换数据类型或错误信息，保持代码流畅。
// 并发 (move): move 关键字对于多线程至关重要，它确保闭包拥有自己所需数据的所有权，避免数据竞争和悬垂引用。
// 组合 (move 捕获): 高阶函数（接收函数返回函数的函数）依赖闭包来“记住”传入的参数（如 f 和 g），从而创建新的功能单元。
## 总结

| 核心概念     | 要点                                                    |
| ------------ | ------------------------------------------------------- |
| 语法         | `\|参数\| 表达式`，支持类型注解和多行                   |
| 三种捕获方式 | `Fn`(不可变借用) → `FnMut`(可变借用) → `FnOnce`(所有权) |
| move 关键字  | 强制获取所有权，跨作用域（多线程）必需                  |
| 作为参数     | 泛型 + `Fn`/`FnMut`/`FnOnce` trait bound                |
| 作为返回值   | `impl Fn(...)` 或 `Box<dyn Fn(...)>`                    |
| 类型推断     | 编译器自动推断，一旦确定不可更改                        |
| vs 函数指针  | 闭包可捕获环境、大小可变；函数指针固定大小、不可捕获    |
```
