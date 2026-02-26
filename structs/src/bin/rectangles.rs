// fn main() {
//     let width = 10;
//     let hgight = 20;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width, hgight)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

/// 表示一个矩形的结构体
/// 包含宽度和高度属性
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// 实现 Rectangle 结构体的方法
impl Rectangle {
    /// 计算并打印矩形宽度与高度之和
    /// 
    /// # 参数
    /// * `self` - 对当前矩形实例的引用
    /// 
    /// # 返回值
    /// 返回宽度与高度的和（u32 类型）
    fn add(&self) -> u32 {
        let count = self.width + self.height;
        println!("add: {}", count);
        self.width + self.height
    }

    /// 计算矩形面积
    /// 
    /// # 参数
    /// * `self` - 对当前矩形实例的引用
    /// 
    /// # 返回值
    /// 返回矩形的面积（width * height，u32 类型）
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

/// 主函数 - 演示如何使用 Rectangle 结构体
/// 创建一个矩形实例，并计算其面积和宽高之和
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("The add of the rectangle is {}.", rect1.add());
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("The rectangle is {:?}.", rect1);
}

/// 计算矩形面积的函数
/// 
/// # 参数
/// * `rectangle` - 对矩形实例的引用
/// 
/// # 返回值
/// 返回矩形的面积（width * height，u32 类型）
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
