use std::fmt::Debug;
use std::fmt::Display;
use trait_generics_lifetime::{NewsArticle, SocialPost, Summary};

#[derive(Debug)]
//#[derive(Debug)] 只能加在结构体（struct）、枚举（enum）或联合体（union）的定义上面，不能加在函数（fn）上面
struct Point<T, U> {
    X: T,
    Y: U,
}
//可以只用一个参数，但是要全部写上
impl<T, U> Point<T, U> {
    fn p(&self) -> &T {
        &self.X
    }
}

//定义方法时也可以为泛型指定限制（constraint）
struct Test<T> {
    value: T,
}
//
//这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，
//而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法。这个方法计算点实例与坐标 (0.0, 0.0) 之间的距离，并使用了只能用于浮点型的数学运算符。
impl Test<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.value.powi(2)).sqrt()
    }
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = latest(&number_list);
    println!("The largest number is {largest}");

    let integer = Point { X: 5, Y: '1' };
    println!("{:?}", integer.X);
    println!("{:?}", integer.Y);
    let x = integer.p();
    println!("{:?}", x);

    let post = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
                               hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", post.summarize());

    let posts = SocialPost {
        username: String::from("John"),
        content: String::from("Hello World!"),
        reply: false,
        repost: false,
    };

    println!("New social post available! {}", posts.summarize_author());

    notify_tow(1, "Hello World!")
}
//抽象出lastest函数

fn latest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

// Trait 作为参数（Trait Bound）
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//使用泛型参数T 作为 Trait bound
pub fn notify_plus<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// where 子句 适合复杂的约束列表 多泛型、多重 Trait 约束（如 $T$ 既要满足 $A$ 也要满足 $B$），或者当关联类型（Associated Types）约束非常长的时候
fn notify_tow<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Debug + Display,
{
    println!("{}", t);
    println!("{}", u);

    return 0;
}

// 语法特性,impl Trait,Trait Bound (<T: Trait>),where 子句
// 可读性,最高（最简洁）,中等,复杂场景下最高
// 灵活性,较低,高,最高
// 强制类型一致,不支持,支持,支持
// 推荐写法,默认首选,涉及多个相同类型参数时,约束逻辑超过 2 个时
