// ==========================================
// 1. 结构体中的泛型
// ==========================================

// 单一泛型结构体：x 和 y 必须是同类型
struct Point<T> {
    x: T,
    y: T,
}
// 为泛型结构体实现方法时，impl 后面也必须声明泛型 <T>,// 为所有 Point<T> 实现方法
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    // 返回引用的元组，生命周期与 &self 绑定（Rust 自动推导）
    fn get(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

// 仅为 Point<f64> 实现特定方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// 多类型参数：x 和 y 可以是不同类型
struct Point2<T, U> {
    x: T,
    y: U,
}


// 方法中使用不同的泛型参数
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// ==========================================
// 2. 枚举中的泛型
// ==========================================

// 【优化】：为了避免与 Rust 自动引入的标准库 Option/Result 冲突，
// 在这里加上 My 前缀，以此来学习其内部原理。
enum MyOption<T> {
    Some(T),
    None,
}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

// 自定义泛型枚举
enum Shape<T> {
    Circle(T),       // 半径
    Rectangle(T, T), // 长、宽
}

// 【修复核心报错】：
// 为泛型 T 增加 Trait Bound（特征约束）：这是泛型编程的灵魂
// 1. Into<f64>: 保证 T 可以安全转换为 f64（例如 i32, f32, f64 都支持）
// 2. Copy: 保证我们可以直接解引用(*radius)获取值，而不会发生所有权转移(Move)报错
impl<T: Into<f64> + Copy> Shape<T> {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => {
                // radius 此时是 &T 类型，*radius 取出 T 的值，再用 .into() 转换为 f64
                let r: f64 = (*radius).into();
                std::f64::consts::PI * r * r
            }
            Shape::Rectangle(length, width) => {
                let l: f64 = (*length).into();
                let w: f64 = (*width).into();
                l * w
            }
        }
    }
}

// ==========================================
// 3. 函数中的泛型
// ==========================================

// 泛型函数：交换两个相同类型的值
fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

// 针对 i32 的非泛型函数
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 针对 f64 的非泛型函数
fn largest_f64(list: &[f64]) -> &f64 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 使用泛型：一个函数适用于所有可比较的类型
// T: PartialOrd 是 Trait Bound，要求 T 必须支持比较大小（即允许使用 `>` 操作符）
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ==========================================
// 主函数测试
// ==========================================
fn main() {
    // --- 结构体测试 ---
    let x = Point { x: 1, y: 2 };
    let y = Point { x: "12", y: "45" }; // T 被推导为 &str
    let mix = Point2 { x: 1, y: "12" }; // T=i32, U=&str

    println!("mix -> x: {}, y: {}", mix.x, mix.y);
    println!("x   -> x: {}, y: {}", x.x, x.y);
    println!("y   -> x: {}, y: {}", y.x, y.y); // 修复了原代码打印标签的笔误

    let point = Point::new(100, 2);
    let res = point.get();
    println!("point.get() -> {:?}", res);

    // --- 函数泛型测试 ---
    println!("swap(2, 1) -> {:?}", swap(2, 1));
    println!("swap(\"12\", \"45\") -> {:?}", swap("12", "45"));

    let numbers = vec![34, 50, 25, 100, 65];
    println!("i32 专用最大数: {}", largest_i32(&numbers));
    println!("泛型函数最大数: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("字符数组最大值: {}", largest(&chars));
    println!("f64 专用最大数: {}", largest_f64(&[1.1, 2.2, 3.3]));

    // --- 修复后的泛型枚举方法测试 ---
    let circle = Shape::Circle(5.0_f64);
    // 这里传入整数，T 被推导为 i32。因为 i32 实现了 Into<f64>，所以可以正常调用 area()
    let rect = Shape::Rectangle(10, 20);

    println!("圆的面积: {:.2}", circle.area());
    println!("矩形的面积: {}", rect.area());
}
