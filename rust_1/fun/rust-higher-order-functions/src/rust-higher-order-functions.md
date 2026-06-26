# Rust 高阶函数学习文档

## 目标

学完本文后，你应该能够理解并使用 Rust 中的高阶函数，包括：

- 把函数或闭包作为参数传入另一个函数
- 使用闭包捕获外部变量
- 区分 `Fn`、`FnMut`、`FnOnce`
- 使用迭代器中的 `map`、`filter`、`fold`
- 返回闭包
- 判断什么时候使用泛型，什么时候使用 `Box<dyn Fn>`

## 1. 什么是高阶函数

高阶函数是指满足以下任意一种条件的函数：

- 接收一个函数作为参数
- 返回一个函数作为结果

在 Rust 里，高阶函数通常和闭包、迭代器、函数指针、`Fn` 系列 trait 一起使用。

一个最简单的例子：

```rust
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

fn main() {
    let result = apply(|x| x * 2, 10);
    println!("{}", result); // 20
}
```

这里 `apply` 接收了一个函数式参数 `f`，然后把 `value` 传给它。

## 2. 函数指针

Rust 中普通函数可以作为参数传递。

```rust
fn double(x: i32) -> i32 {
    x * 2
}

fn apply(f: fn(i32) -> i32, value: i32) -> i32 {
    f(value)
}

fn main() {
    let result = apply(double, 10);
    println!("{}", result); // 20
}
```

这里的 `fn(i32) -> i32` 是函数指针类型。

函数指针适合只接收普通函数的场景，但实际开发中更常用泛型配合 `Fn`，因为它既能接收普通函数，也能接收闭包。

```rust
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("{}", apply(double, 10));
    println!("{}", apply(|x| x + 1, 10));
}
```

## 3. 闭包

闭包是匿名函数，可以直接赋值给变量，也可以作为参数传入函数。

```rust
fn main() {
    let add_one = |x| x + 1;

    println!("{}", add_one(10)); // 11
}
```

闭包参数和返回值通常可以由编译器推断。也可以显式标注类型：

```rust
fn main() {
    let add_one = |x: i32| -> i32 {
        x + 1
    };

    println!("{}", add_one(10));
}
```

闭包还可以捕获外部变量：

```rust
fn main() {
    let factor = 3;

    let multiply = |x| x * factor;

    println!("{}", multiply(10)); // 30
}
```

这里 `multiply` 捕获了外部变量 `factor`。

## 4. `Fn`、`FnMut`、`FnOnce`

Rust 用三个 trait 来描述闭包如何使用捕获的变量。

### 4.1 `Fn`

`Fn` 表示闭包只读捕获环境，不修改也不消费捕获的变量。

```rust
fn call_twice<F>(f: F)
where
    F: Fn(),
{
    f();
    f();
}

fn main() {
    let name = String::from("Rust");

    call_twice(|| {
        println!("{}", name);
    });
}
```

这个闭包只是读取 `name`，所以它实现了 `Fn`。

### 4.2 `FnMut`

`FnMut` 表示闭包可能会修改捕获的变量。

```rust
fn call_twice<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
}

fn main() {
    let mut count = 0;

    call_twice(|| {
        count += 1;
        println!("{}", count);
    });
}
```

这里闭包修改了 `count`，所以它需要 `FnMut`。

注意函数参数里要写 `mut f: F`，因为调用 `FnMut` 闭包需要可变地调用它。

### 4.3 `FnOnce`

`FnOnce` 表示闭包可能会消费捕获的变量，因此只能调用一次。

```rust
fn call_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn main() {
    let text = String::from("hello");

    call_once(|| {
        drop(text);
    });
}
```

这里闭包把 `text` 移动进 `drop`，所以它消费了 `text`，只能调用一次。

### 4.4 三者关系

可以简单记住：

```text
Fn      不修改、不消费捕获变量
FnMut   可能修改捕获变量
FnOnce  可能消费捕获变量
```

能力包含关系可以理解为：

```text
Fn 可以当作 FnMut 使用
FnMut 可以当作 FnOnce 使用
```

也就是说，要求越宽松，能传入的闭包越多。`FnOnce` 最宽松，因为它只要求闭包能被调用一次。

## 5. 高阶函数和迭代器

Rust 最常见的高阶函数来自迭代器。

### 5.1 `map`

`map` 用来把每个元素转换成另一个值。

```rust
fn main() {
    let nums = vec![1, 2, 3];

    let doubled: Vec<i32> = nums
        .iter()
        .map(|x| x * 2)
        .collect();

    println!("{:?}", doubled); // [2, 4, 6]
}
```

注意：`iter()` 产生的是引用，所以这里 `x` 的类型是 `&i32`。Rust 会自动解引用，所以 `x * 2` 可以正常工作。

如果使用 `into_iter()`，会消费原集合：

```rust
fn main() {
    let nums = vec![1, 2, 3];

    let doubled: Vec<i32> = nums
        .into_iter()
        .map(|x| x * 2)
        .collect();

    println!("{:?}", doubled);
}
```

### 5.2 `filter`

`filter` 用来筛选元素。

```rust
fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    let evens: Vec<i32> = nums
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect();

    println!("{:?}", evens); // [2, 4]
}
```

### 5.3 `fold`

`fold` 用来把多个元素累积成一个结果。

```rust
fn main() {
    let nums = vec![1, 2, 3, 4];

    let sum = nums
        .into_iter()
        .fold(0, |acc, x| acc + x);

    println!("{}", sum); // 10
}
```

`fold` 的两个核心参数：

- 初始值：这里是 `0`
- 累积函数：这里是 `|acc, x| acc + x`

再看一个计算乘积的例子：

```rust
fn main() {
    let nums = vec![2, 3, 4];

    let product = nums
        .into_iter()
        .fold(1, |acc, x| acc * x);

    println!("{}", product); // 24
}
```

## 6. 自己写高阶函数

下面写一个函数，用来转换 `Vec<i32>` 中的每个元素：

```rust
fn transform_vec<F>(items: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    items.into_iter().map(f).collect()
}

fn main() {
    let nums = vec![1, 2, 3];

    let squares = transform_vec(nums, |x| x * x);

    println!("{:?}", squares); // [1, 4, 9]
}
```

这里 `transform_vec` 本身不关心具体怎么转换，只关心你传进来的闭包能把 `i32` 转成 `i32`。

再写一个把函数连续应用两次的例子：

```rust
fn apply_twice<F>(mut f: F, value: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    let first = f(value);
    f(first)
}

fn main() {
    let result = apply_twice(|x| x * 2, 5);

    println!("{}", result); // 20
}
```

这里使用 `FnMut`，是因为它比 `Fn` 更宽松，允许传入可能会修改内部状态的闭包。

## 7. 返回闭包

函数也可以返回闭包。最常见的写法是 `impl Fn`。

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn main() {
    let add_five = make_adder(5);

    println!("{}", add_five(10)); // 15
}
```

这里需要 `move`，因为返回的闭包要在 `make_adder` 结束后继续使用 `n`。

如果没有 `move`，闭包可能只是借用 `n`，而 `n` 在函数结束时就被释放了。

再看一个返回乘法器的例子：

```rust
fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

fn main() {
    let times_three = make_multiplier(3);

    println!("{}", times_three(10)); // 30
}
```

## 8. `impl Fn` 和 `Box<dyn Fn>`

如果一个函数只返回一种闭包类型，优先使用 `impl Fn`。

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
```

如果不同分支返回不同闭包类型，就需要使用 trait object，比如 `Box<dyn Fn>`。

```rust
fn make_operator(kind: &str) -> Box<dyn Fn(i32) -> i32> {
    match kind {
        "double" => Box::new(|x| x * 2),
        "square" => Box::new(|x| x * x),
        _ => Box::new(|x| x),
    }
}

fn main() {
    let op = make_operator("square");

    println!("{}", op(5)); // 25
}
```

原因是不同闭包即使签名一样，它们的具体类型也不同。`Box<dyn Fn>` 可以把不同闭包装进同一种类型里。

## 9. 常见选择

### 9.1 参数优先用泛型

大多数情况下，函数参数应该这样写：

```rust
fn run<F>(f: F)
where
    F: Fn(),
{
    f();
}
```

这种写法是静态分发，通常性能更好，也更符合 Rust 的常见风格。

### 9.2 需要存储不同闭包时用 `Box<dyn Fn>`

如果要把不同闭包装进一个集合，就需要 trait object。

```rust
fn main() {
    let tasks: Vec<Box<dyn Fn()>> = vec![
        Box::new(|| println!("task 1")),
        Box::new(|| println!("task 2")),
    ];

    for task in tasks {
        task();
    }
}
```

### 9.3 不确定用哪个 trait 时

可以按下面的顺序判断：

```text
闭包会消费捕获的变量吗？
是：用 FnOnce
否：继续判断

闭包会修改捕获的变量吗？
是：用 FnMut
否：用 Fn
```

如果只是接收一个函数并调用一次，`FnOnce` 是最宽松的选择。

如果你需要多次调用闭包，并且闭包可能修改状态，用 `FnMut`。

如果你需要多次调用闭包，并且闭包只读状态，用 `Fn`。

## 10. 常见错误

### 10.1 返回闭包时忘记 `move`

错误示例：

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    |x| x + n
}
```

修正：

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
```

返回闭包时，如果闭包使用了函数内部变量，通常需要 `move`。

### 10.2 需要 `FnMut` 却写成 `Fn`

错误示例：

```rust
fn call_twice<F>(f: F)
where
    F: Fn(),
{
    f();
    f();
}

fn main() {
    let mut count = 0;

    call_twice(|| {
        count += 1;
    });
}
```

修正：

```rust
fn call_twice<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
}
```

因为闭包修改了捕获变量 `count`，所以它需要 `FnMut`。

### 10.3 `impl Fn` 不能返回多个不同闭包类型

错误示例：

```rust
fn make_operator(kind: &str) -> impl Fn(i32) -> i32 {
    match kind {
        "double" => |x| x * 2,
        "square" => |x| x * x,
        _ => |x| x,
    }
}
```

修正：

```rust
fn make_operator(kind: &str) -> Box<dyn Fn(i32) -> i32> {
    match kind {
        "double" => Box::new(|x| x * 2),
        "square" => Box::new(|x| x * x),
        _ => Box::new(|x| x),
    }
}
```

## 11. 练习

### 练习 1：实现 `apply`

实现一个函数，接收一个闭包和一个数字，把数字传给闭包。

要求：

```rust
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    // TODO
}
```

目标效果：

```rust
fn main() {
    let result = apply(|x| x + 10, 5);
    println!("{}", result); // 15
}
```

### 练习 2：实现 `apply_twice`

实现一个函数，把闭包连续应用两次。

目标效果：

```rust
fn main() {
    let result = apply_twice(|x| x * 2, 5);
    println!("{}", result); // 20
}
```

### 练习 3：使用 `map`

把下面的 `Vec<String>` 转成每个字符串的长度：

```rust
fn main() {
    let words = vec![
        String::from("rust"),
        String::from("closure"),
        String::from("iterator"),
    ];

    // 目标结果：[4, 7, 8]
}
```

### 练习 4：使用 `filter`

筛选出长度大于 4 的字符串：

```rust
fn main() {
    let words = vec!["rust", "book", "closure", "iterator"];

    // 目标结果：["closure", "iterator"]
}
```

### 练习 5：使用 `fold`

用 `fold` 计算数组中所有数字的乘积：

```rust
fn main() {
    let nums = vec![2, 3, 4];

    // 目标结果：24
}
```

### 练习 6：返回闭包

实现 `make_multiplier`：

```rust
fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    // TODO
}

fn main() {
    let times_four = make_multiplier(4);
    println!("{}", times_four(6)); // 24
}
```

## 12. 参考答案

### 练习 1

```rust
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}
```

### 练习 2

```rust
fn apply_twice<F>(mut f: F, value: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    let first = f(value);
    f(first)
}
```

### 练习 3

```rust
fn main() {
    let words = vec![
        String::from("rust"),
        String::from("closure"),
        String::from("iterator"),
    ];

    let lengths: Vec<usize> = words
        .iter()
        .map(|word| word.len())
        .collect();

    println!("{:?}", lengths);
}
```

### 练习 4

```rust
fn main() {
    let words = vec!["rust", "book", "closure", "iterator"];

    let long_words: Vec<&str> = words
        .into_iter()
        .filter(|word| word.len() > 4)
        .collect();

    println!("{:?}", long_words);
}
```

### 练习 5

```rust
fn main() {
    let nums = vec![2, 3, 4];

    let product = nums
        .into_iter()
        .fold(1, |acc, x| acc * x);

    println!("{}", product);
}
```

### 练习 6

```rust
fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}
```

## 13. 学习路线

建议按下面顺序学习：

1. 先理解闭包语法：`|x| x + 1`
2. 再理解闭包如何捕获外部变量
3. 然后学习 `Fn`、`FnMut`、`FnOnce`
4. 接着练习迭代器的 `map`、`filter`、`fold`
5. 最后学习返回闭包和 `Box<dyn Fn>`

掌握高阶函数之后，你会更容易理解 Rust 迭代器、回调、配置式 API、异步任务和函数式编程风格。
