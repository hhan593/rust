# Rust Pin 与 Unpin 知识手册

## 一、为什么需要 Pin？

### 1.1 自引用类型问题

Pin 解决的根本问题与**自引用类型（Self-Referential Types）**有关。自引用类型是包含指向自身数据指针的结构体：

```rust
struct SelfRef {
    value: String,
    ptr: *const String, // 指向自身的 value 字段
}
```

如果这样的结构体在内存中被**移动（move）**，`ptr` 仍然指向旧地址，导致**悬垂指针（dangling pointer）**，这是未定义行为。

### 1.2 异步编程中的自引用

Rust 中的 `async` 块会被编译器转换为状态机（实现 `Future` trait 的结构体）。这些状态机**可能是自引用的**，例如：

```rust
async fn example() {
    let data = vec![1, 2, 3];
    let reference = &data; // 跨 await 点的引用 → 编译后变成自引用
    some_async_fn().await;
    println!("{:?}", reference);
}
```

编译后，`data` 和 `reference` 都是状态机结构体的字段，`reference` 指向 `data`，形成自引用。因此 **Pin 对异步编程至关重要**。

---

## 二、Pin 是什么？

### 2.1 基本定义

`Pin` **不是 trait，而是一个结构体**，定义在 `std::pin` 模块中：

```rust
pub struct Pin<Ptr> {
    pointer: Ptr, // 私有字段，无法直接访问
}
```

`Pin` 包裹一个指针类型（如 `&mut T`、`Box<T>`），确保该指针指向的数据**不会被移动**：

- `Pin<&mut T>` — 固定可变引用指向的数据
- `Pin<&T>` — 固定不可变引用指向的数据
- `Pin<Box<T>>` — 固定堆上数据

### 2.2 核心保证

Pin 的核心保证是：**被 Pin 住的数据，在其生命周期内不会被移动到其他内存位置**（除非该类型实现了 `Unpin`）。

---

## 三、Unpin 是什么？

### 3.1 基本定义

`Unpin` 是一个 **auto trait**（自动特征），表明一个类型**即使被 Pin 住也可以安全移动**：

```rust
pub auto trait Unpin {}
```

### 3.2 默认行为

编译器**默认给所有类型实现 `Unpin`**。以下类型都是 `Unpin` 的：

- 所有基本类型：`bool`、`i32`、`f64`、`char` 等
- 引用类型：`&T`、`&mut T`
- 标准库类型：`Box<T>`、`String`、`Vec<T>` 等

### 3.3 `!Unpin` 类型

只有少数类型是 `!Unpin`（即没有实现 `Unpin`）：

| 类型 | 说明 |
|------|------|
| `std::marker::PhantomPinned` | 标记类型，用于手动标记 `!Unpin` |
| 包含 `PhantomPinned` 的类型 | 传递性 |
| `async` 块生成的 Future | 编译器自动标记为 `!Unpin` |

---

## 四、Pin 与 Unpin 的交互规则

**核心规则：Pin 和 Unpin 互相抵消。**

| 条件 | 效果 |
|------|------|
| `T: Unpin`（默认情况） | `Pin<P<T>>` 等同于 `P<T>`，Pin 无实际效果 |
| `T: !Unpin` | `Pin<P<T>>` 会真正阻止 `T` 被移动 |

```rust
// T: Unpin 时，Pin 没有额外限制
let mut val = 42i32; // i32: Unpin
let pinned = Pin::new(&mut val); // 完全安全，等同于 &mut i32

// T: !Unpin 时，Pin 才有意义
use std::marker::PhantomPinned;
struct Unmovable {
    data: String,
    _pin: PhantomPinned, // 使类型变为 !Unpin
}
```

---

## 五、如何创建 Pin

### 5.1 Pin::new（安全，要求 T: Unpin）

```rust
let mut x = 5;
let pinned = Pin::new(&mut x); // 安全：i32 是 Unpin 的
```

### 5.2 Pin::new_unchecked（不安全，用于 !Unpin 类型）

```rust
use std::marker::PhantomPinned;

struct MyStruct {
    data: String,
    _pin: PhantomPinned,
}

let mut val = MyStruct {
    data: String::from("hello"),
    _pin: PhantomPinned,
};

// unsafe：你必须保证 val 不会被移动
let pinned = unsafe { Pin::new_unchecked(&mut val) };
```

### 5.3 Box::pin（安全，堆上固定）

```rust
use std::marker::PhantomPinned;

struct MyStruct {
    data: String,
    _pin: PhantomPinned,
}

// 安全：数据在堆上，Box 保证不会移动
let pinned: Pin<Box<MyStruct>> = Box::pin(MyStruct {
    data: String::from("hello"),
    _pin: PhantomPinned,
});
```

### 5.4 pin! 宏（栈上固定，Rust 1.68+）

```rust
use std::pin::pin;

let pinned = pin!(SomeStruct::new()); // 栈上固定
```

---

## 六、栈固定 vs 堆固定

### 6.1 栈上固定（Stack Pinning）

```rust
use std::pin::pin;
use std::marker::PhantomPinned;

struct Test {
    value: i32,
    _pin: PhantomPinned,
}

let pinned = pin!(Test { value: 42, _pin: PhantomPinned });
// pinned 的类型是 Pin<&mut Test>
// Test 被固定在栈上，不会被移动
```

**特点：**
- 使用 `pin!` 宏（推荐）或手动 `unsafe`
- 数据在栈上，生命周期受限于当前作用域
- 无堆分配开销

### 6.2 堆上固定（Heap Pinning）

```rust
let pinned: Pin<Box<Test>> = Box::pin(Test {
    value: 42,
    _pin: PhantomPinned,
});
// Test 被固定在堆上
```

**特点：**
- 使用 `Box::pin()` 创建
- 数据在堆上，可以安全地在不同作用域间传递
- 有堆分配开销
- 更灵活，是异步编程中的常见选择

---

## 七、自引用结构体示例

### 7.1 问题演示

```rust
#[derive(Debug)]
struct SelfRef {
    value: String,
    ptr: *const String,
}

impl SelfRef {
    fn new(txt: &str) -> Self {
        SelfRef {
            value: String::from(txt),
            ptr: std::ptr::null(), // 初始为空
        }
    }

    fn init(&mut self) {
        self.ptr = &self.value as *const String;
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn ptr(&self) -> &String {
        assert!(!self.ptr.is_null(), "尚未初始化");
        unsafe { &*self.ptr }
    }
}

fn main() {
    let mut a = SelfRef::new("hello");
    a.init();

    let mut b = SelfRef::new("world");
    b.init();

    println!("a: value={}, ptr={}", a.value(), a.ptr()); // hello, hello

    // 交换 a 和 b 后，ptr 指向错误的位置！
    std::mem::swap(&mut a, &mut b);
    println!("a: value={}, ptr={}", a.value(), a.ptr()); // world, hello ← 错误！
}
```

### 7.2 使用 Pin 解决

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

struct SelfRef {
    value: String,
    ptr: *const String,
    _pin: PhantomPinned, // 标记为 !Unpin
}

impl SelfRef {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let mut s = Box::pin(SelfRef {
            value: String::from(txt),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });

        // 安全地初始化自引用指针
        let ptr = &s.value as *const String;
        unsafe {
            let mut_ref = Pin::as_mut(&mut s);
            Pin::get_unchecked_mut(mut_ref).ptr = ptr;
        }

        s
    }

    fn value(self: Pin<&Self>) -> &str {
        &self.get_ref().value
    }

    fn ptr(self: Pin<&Self>) -> &String {
        unsafe { &*self.get_ref().ptr }
    }
}

fn main() {
    let a = SelfRef::new("hello");
    let b = SelfRef::new("world");

    println!("a: value={}, ptr={}", a.as_ref().value(), a.as_ref().ptr());
    // hello, hello ✓

    // std::mem::swap(&mut a, &mut b); // 编译错误！无法获取 &mut T
}
```

---

## 八、在异步编程中的应用

### 8.1 Future::poll 签名

```rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

`poll` 方法接收 `Pin<&mut Self>` 而不是 `&mut self`，因为 Future 可能是自引用的。

### 8.2 实现一个简单的 Future

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture {
    value: i32,
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.value * 2)
    }
}
```

### 8.3 使用 pin-project 进行安全投影

当你的 Future 包含其他 Future 作为字段时，需要**Pin 投影（Pin Projection）**：

```rust
use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

#[pin_project]
pub struct TimedWrapper<Fut: Future> {
    start: Option<Instant>,   // 不需要 pin
    #[pin]                     // 需要 pin（因为是 Future）
    future: Fut,
}

impl<Fut: Future> Future for TimedWrapper<Fut> {
    type Output = (Fut::Output, std::time::Duration);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project(); // 安全的 pin 投影
        let start = this.start.get_or_insert_with(Instant::now);
        let inner_poll = this.future.poll(cx); // this.future 是 Pin<&mut Fut>

        match inner_poll {
            Poll::Pending => Poll::Pending,
            Poll::Ready(output) => {
                let elapsed = start.elapsed();
                Poll::Ready((output, elapsed))
            }
        }
    }
}
```

### 8.4 常见的异步函数签名约束

```rust
// 要求 Future 是 Unpin 的（可以安全移动）
async fn process<F: Future<Output = i32> + Unpin>(fut: F) -> i32 {
    fut.await
}

// 使用 Pin<Box<dyn Future>> 处理 !Unpin 的 Future
async fn process_boxed(fut: Pin<Box<dyn Future<Output = i32>>>) -> i32 {
    fut.await
}

// 使用 Box::pin 将 !Unpin 的 Future 转为 Unpin
async fn example() {
    let fut = async { 42 }; // !Unpin
    let boxed = Box::pin(fut); // Pin<Box<...>> 是 Unpin 的
    let result = process(boxed).await;
}
```

---

## 九、Pin 的常用方法速查

| 方法 | 约束 | 说明 |
|------|------|------|
| `Pin::new(ptr)` | `T: Unpin` | 安全创建 Pin |
| `Pin::new_unchecked(ptr)` | 无 | 不安全创建 Pin |
| `Pin::as_ref(&self)` | 无 | 获取 `Pin<&T>` |
| `Pin::as_mut(&mut self)` | 无 | 获取 `Pin<&mut T>` |
| `Pin::get_ref(&self)` | 无 | 获取 `&T` |
| `Pin::get_mut(self)` | `T: Unpin` | 安全获取 `&mut T` |
| `Pin::get_unchecked_mut(self)` | 无 | 不安全获取 `&mut T` |
| `Pin::into_inner(self)` | `T: Unpin` | 取出内部指针 |
| `Pin::into_inner_unchecked(self)` | 无 | 不安全取出内部指针 |

---

## 十、常见模式与最佳实践

### 10.1 何时需要关心 Pin？

1. **实现 `Future` trait** — `poll` 方法要求 `Pin<&mut Self>`
2. **手动实现包含 Future 字段的结构体** — 需要 Pin 投影
3. **使用自引用数据结构** — 如侵入式链表
4. **编写异步运行时或底层异步库** — 需要手动管理 Future 的内存位置

### 10.2 何时不需要关心 Pin？

1. **仅使用 `async/await` 语法** — 编译器自动处理
2. **使用标准库的高层异步 API** — 已封装好 Pin 细节
3. **普通的同步 Rust 代码** — 所有类型默认 `Unpin`

### 10.3 推荐做法

```rust
// ✅ 使用 Box::pin 进行堆上固定（最常用）
let pinned_future = Box::pin(my_async_fn());

// ✅ 使用 pin! 宏进行栈上固定
let pinned = std::pin::pin!(my_value);

// ✅ 使用 pin-project crate 进行安全投影
#[pin_project]
struct MyWrapper<F: Future> {
    #[pin]
    inner: F,
}

// ❌ 避免手动使用 unsafe Pin 操作（除非你确切知道在做什么）
// ❌ 避免对 Unpin 类型使用 Pin（没有意义）
```

---

## 十一、Pin 的其他应用场景

### 11.1 侵入式集合（Intrusive Collections）

侵入式链表中，元素包含指向相邻元素的指针。如果某个元素被移动，其他元素指向它的指针就会失效。Pin 可以防止这种情况。

### 11.2 FFI 场景

与 C 代码交互时，如果 C 代码持有指向 Rust 数据的指针，需要确保 Rust 数据不会被移动。

---

## 参考资料

- [Rust 官方文档 - std::pin](https://doc.rust-lang.org/std/pin/)
- [Rust 语言圣经 - Pin 和 Unpin](https://course.rs/advance/async/pin-unpin.html)
- [Cloudflare Blog - Pin, Unpin, and why Rust needs them](https://blog.cloudflare.com/pin-and-unpin-in-rust/)
- [Folyd - Rust 的 Pin 与 Unpin](https://folyd.com/blog/rust-pin-unpin/)
- [Rust Pin 进阶](https://rustmagazine.github.io/rust_magazine_2021/chapter_12/rust-pin-advanced.html)
- [fasterthanli.me - Pin and suffering](https://fasterthanli.me/articles/pin-and-suffering)
- [Cryptical - Pin & Unpin](https://cryptical.xyz/rust/pin-unpin)
