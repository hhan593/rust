// Trait 定义了一组方法签名，描述某种类型应该具有的行为。类似于其他语言中的接口（interface），但功能更强大。
use std::fmt::Display;
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    title: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}
struct Tweet {
    username: String,
    content: String,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        title: String::from("Rust Programming Language"),
        author: String::from("John Doe"),
        content: String::from("Rust is a systems programming language..."),
    };
    println!("{}", article.summarize());
    let tweet = Tweet {
        username: String::from("alice"),
        content: String::from("Hello, world!"),
    };
    println!("{}", tweet.summarize());

    //2.7  Trait 对象与动态分发（dyn Trait）
    // trait 对象：通过引用或 Box 持有
    /// 接受一个动态分发的 `Summary` trait 对象，并打印其摘要信息。
    ///
    /// 该函数使用 `dyn Summary` 作为参数类型，实现了动态分发（Dynamic Dispatch）。
    /// 这意味着在编译时不需要知道具体的类型，只要该类型实现了 `Summary` trait 即可。
    /// 适用于需要处理多种不同实现了相同 trait 的类型的场景。
    ///
    /// # 参数
    ///
    /// * `item` - 一个对 `Summary` trait 的动态引用（`&dyn Summary`）。任何实现了
    ///   `Summary` trait 的类型的引用都可以作为参数传入。
    ///
    /// # 示例
    ///
    pub fn notify_dynamic(item: &dyn Summary) {
        println!("Breaking news! {}", item.     ());
    }

    let articles: Vec<Box<dyn Summary>> = vec![
        Box::new(NewsArticle {
            title: String::from("Rust Programming Language"),
            author: String::from("John Doe"),
            content: String::from("Rust is a systems programming language..."),
        }),
        Box::new(Tweet {
            username: String::from("alice"),
            content: String::from("Hello, world!"),
        }),
    ];
    for article in &articles {
        println!("{}", article.summarize());
    }
}
// **孤儿规则（Orphan Rule）**：只有当 trait 或类型至少有一个定义在当前 crate 中时，才能为该类型实现该 trait。不能为外部类型实现外部 trait，只能为自己的。
//trait作为参数

// 语法 1：impl Trait（语法糖，简洁）
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// 语法 2：泛型 + Trait Bound（更灵活）
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// 语法 3：where 子句（适合复杂约束）
pub fn notify3<T>(item: &T)
where
    T: Summary,
{
    println!("Breaking news! {}", item.summarize());
}

// 多重 Trait 约束
// 使用 + 语法
pub fn notify4(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}
// Trait Bound 形式
pub fn notify5<T: Summary + Display>(item: &T) {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone,
{
    1
}
//Trait 作为返回值

// 返回实现了某 Trait 的类型,`impl Trait` 返回类型只能返回**单一具体类型**。
// 本函数返回了实现Summary特性的内容
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("www"),
        content: String::from("of course, as you probably already know, people"),
    }
}
// 下面这个函数是错误的，因为没有办法返回两个类型，因为只实现返回了Summary的
// 编译错误！不能返回不同的具体类型
fn returns_summarizable2(switch: bool) -> impl Summary {
    if switch {
        // NewsArticle { /* ... */ } // 类型 A
    } else {
        // Tweet { /* ... */ } // 类型 B  ← 不兼容
    }
    Tweet {
        username: String::from("111"),
        content: String::from("of course, as you probably already know, people"),
    }
}
