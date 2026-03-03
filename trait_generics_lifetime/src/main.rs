use std::fmt::{Debug, Display};

// --- 模拟你本地库中的 Trait 定义 ---
pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
// ------------------------------------

#[derive(Debug)]
struct Point<T, U> {
    x: T, // 推荐使用小写
    y: U,
}

impl<T, U> Point<T, U> {
    fn p(&self) -> &T {
        &self.x
    }
}

struct Test<T> {
    value: T,
}

// 特化实现：只有当 T 是 f32 时，Test 结构体才拥有这个方法
impl Test<f32> {
    fn distance_from_origin(&self) -> f32 {
        self.value.powi(2).sqrt()
    }
}

fn main() {
    // 1. 测试数组最大值 (使用泛型版本)
    let number_list = vec![34, 50, 25, 100, 65];
    if let Some(largest) = latest(&number_list) {
        println!("The largest number is {largest}");
    }

    // 2. 测试泛型结构体 Point
    let integer = Point { x: 5, y: '1' };
    println!("Point: x = {:?}, y = {:?}", integer.x, integer.y);
    let x_ref = integer.p();
    println!("Value from method p: {:?}", x_ref);

    // 3. 测试 NewsArticle (Trait 对象用法)
    let post = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins are the best."),
    };
    println!("New article: {}", post.summarize());

    // 4. 测试 SocialPost
    let posts = SocialPost {
        username: String::from("John"),
        content: String::from("Hello World!"),
        reply: false,
        repost: false,
    };
    println!("Author: {}", posts.summarize_author());

    // 5. 调用复杂的 where 约束函数
    // 传入的第一个参数 &1 满足 Display + Clone
    // 第二个参数 &"Hello" 满足 Clone + Debug + Display
    notify_tow(&1, &"Hello World!");
}

/// 泛型版本的获取最大值函数
/// 约束 T 必须实现 PartialOrd（支持比较）和 Copy（方便移动值）
fn latest<T: PartialOrd + Copy>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    Some(largest)
}

// Trait 作为参数 (语法糖形式)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound 形式
pub fn notify_plus<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// where 子句：适合处理复杂或多个泛型的约束
fn notify_tow<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug + Display,
{
    println!("Data T: {}", t);
    println!("Data U: {}", u);
    0
}

// 语法特性,impl Trait,Trait Bound (<T: Trait>),where 子句
// 可读性,最高（最简洁）,中等,复杂场景下最高
// 灵活性,较低,高,最高
// 强制类型一致,不支持,支持,支持
// 推荐写法,默认首选,涉及多个相同类型参数时,约束逻辑超过 2 个时
