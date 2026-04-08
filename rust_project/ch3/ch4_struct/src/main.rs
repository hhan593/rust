// 定义一个枚举类型，表示饮品的口味
enum Flavor {
    Spicy,   // 辣味
    Sweet,   // 甜味
    Fruity,  // 果味
}

// 定义一个结构体，表示饮品
struct Drink {
    flavor: Flavor, // 饮品的口味
    price: f64,     // 饮品的价格
}

// 为 Drink 结构体实现方法和关联函数
impl Drink {
    // 关联常量：定义饮品的最高价格限制
    const MAX_PRICE: f64 = 10.0;

    // 方法：模拟购买饮品的行为
    // 注意：这里使用 &self，表示借用实例，而不是获取所有权
    fn buy(&self) {
        // 如果价格超过了最大限制
        if self.price > Drink::MAX_PRICE {
            println!(" I am poor"); // 输出 "I am poor"
            ret flavor, price

    // 关联函数：构造函数，用于创建新的 Drink 实例
    // 参数：price (价格), flavor (口味)
    // 返回：Self (即 Drink 类型的新实例)
    fn new(price: f64, flavor: Flavor) -> Self {
        Drink {
            flavor,
            price,
        }
    }
}

// 定义一个函数，用于打印饮品的信息
// 注意：这里参数是 Drink，意味着函数会获取 drink 的所有权 (Move)
fn print_drink(drink: Drink) {
    // 使用 match 表达式匹配饮品的口味
    match drink.flavor {
        Flavor::Fruity => println!("fruity"),
        Flavor::Spicy => println!("spicy"),
        Flavor::Sweet => println !("sweet"),
    }
    // 打印饮品的价格
    println!("{}", drink.price);
}

fn main() {
    // 1. 使用结构体更新语法直接创建实例
    let sweet = Drink {
        flavor: Flavor::Sweet,
        price: 6.0,
    };
    
    // 打印价格
    println!("{}", sweet.price);
    
    // 调用 print_drink 函数
    // 注意：由于 print_drink 获取了所有权，sweet 变量在此行之后将失效
    print_drink(sweet); 

    // 2. 变量遮蔽：创建一个新的变量也叫 sweet，覆盖了上面的变量
    // 使用关联函数 new 创建实例，价格为 12.0，口味为 Spicy
    let sweet = Drink::new(12.0, Flavor::Spicy);
    
    // 调用 buy 方法
    // 因为 12.0 > MAX_PRICE (10.0)，所以会输出 "I am poor"
    sweet.buy();
}