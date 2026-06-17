// 【知识点总结】
// 1. 结构体 (Struct) 与 枚举 (Enum)：用于定义包含异构数据或分类状态的自定义数据类型。
// 2. 实现块 (impl block)：Rust 将数据定义 (struct) 与行为定义 (impl) 分离。在 impl 块中可以定义与该类型相关的函数和常量。
// 3. 关联函数 (Associated Function)：不以 self 作为第一个参数的函数，类似于其他语言的静态方法，常用于构造器（如 new）。调用方式为 `StructName::function_name()`。
// 4. 实例方法 (Method)：以 `&self`, `&mut self` 或 `self` 作为第一个参数的函数。通过实例加点号调用（如 `instance.method_name()`）。
// 5. 关联常量 (Associated Constant)：在 impl 块中定义的常量，属于类型本身而不是实例，调用方式为 `StructName::CONSTANT_NAME`。
// 6. Self 关键字：在 impl 块中，`Self`（大写 S）是当前实现块对应类型的别名。

struct Person {                                  // 定义名为 Person 的结构体，用于聚合个人的基本信息 (知识点：结构体定义)
    name: String,                                // 声明 name 字段，类型为基于堆的动态字符串 String (知识点：结构体字段，动态字符串)
    age: i32,                                    // 声明 age 字段，类型为 32 位有符号整型 i32 (知识点：结构体字段，基础数据类型)
}                                                // 结构体作用域结束 (知识点：语法与作用域)

// 结构体是一种用户定义的数据类型，用于创建自定义的数据结构，每条数据称为属性（字段） 通过点（.）来访问结构体的属性 // 原代码注释

impl Person {                                    // 为 Person 结构体开启一个实现块，绑定相关行为 (知识点：impl 块)
    //关联函数是类型相关联的函数，调用时为结构体名：：函数名 // 原代码注释
    fn new(name: String, age: i32) -> Self {     // 定义关联函数 new，接收名字和年龄，返回 Self（即 Person 本身） (知识点：关联函数，构造器模式，Self 关键字)
        Self { name, age }                       // 实例化并返回 Person 对象。这里使用了字段初始化简写语法（当变量名与字段名相同时可省略缩写）(知识点：结构体实例化，字段初始化简写)
    }                                            // new 函数作用域结束 (知识点：作用域)

    //属性方法,通过实例调用，（&self,&mut self,self） // 原代码注释
    fn print_person(&self, other: &Person) {     // 定义实例方法 print_person，接收自身的不可变引用 &self 和另一个 Person 对象的引用 &other (知识点：实例方法，借用/引用)
        let name = &self.name;                   // 将自身 name 字段的引用绑定到局部变量 name 上 (知识点：访问结构体字段，引用赋值)
        let age = &self.age;                     // 将自身 age 字段的引用绑定到局部变量 age 上 (知识点：访问结构体字段，引用赋值)
        println!("{}", other.age);               // 打印传入的 other 对象的 age 字段 (知识点：通过引用访问结构体字段并打印)

        // 为什么不支持 println!("{}", other);       // 原代码注释（解答见下方）
        // (知识点：格式化输出 trait。因为 Person 结构体默认没有实现 std::fmt::Display 或 std::fmt::Debug 特型，编译器不知道如何将完整的自定义结构体格式化为字符串)

        println!("name {} age {}", name, age);   // 打印当前实例的 name 和 age (知识点：格式化输出宏 println!)
    }                                            // print_person 方法结束 (知识点：作用域)

    // 和结构体类型相关的变量，也可以在特质trait和枚举中,调用的时候使用结构体::PI // 原代码注释
    const PI: f64 = 3.14;                        // 在 impl 块中定义一个关联常量 PI，类型为 f64 (知识点：关联常量 Associated Constants)
}                                                // Person 的 impl 块结束 (知识点：作用域)

enum Flavor {                                    // 定义名为 Flavor 的枚举，表示饮料的不同风味 (知识点：枚举 enum 定义)
    Sweet,                                       // 定义变体 Sweet (甜味) (知识点：无数据枚举变体)
    Fruity,                                      // 定义变体 Fruity (果味) (知识点：无数据枚举变体)
    Spicy,                                       // 定义变体 Spicy (辣味) (知识点：无数据枚举变体)
}                                                // Flavor 枚举作用域结束 (知识点：作用域)

struct Drink {                                   // 定义名为 Drink 的结构体，表示饮料信息 (知识点：复合数据结构定义)
    flavor: Flavor,                              // 声明 flavor 字段，其类型为我们刚刚自定义的 Flavor 枚举 (知识点：自定义枚举作为结构体字段)
    price: f64,                                  // 声明 price 字段，类型为 64 位浮点数 (知识点：结构体字段，浮点类型)
}                                                // Drink 结构体作用域结束 (知识点：作用域)

// 打印饮料信息的函数                               // 原代码注释
// 参数 drink: 一个 Drink 类型的结构体，包含饮料的风味和价格信息 // 原代码注释
fn print_drink(drink: Drink) {                   // 定义独立函数 print_drink，按值接收 Drink 对象，发生所有权转移 (Move) (知识点：函数定义，所有权转移)
    // 使用 match 语句根据饮料的风味类型打印不同的信息     // 原代码注释
    match drink.flavor {                         // 使用 match 表达式匹配 drink 实例的 flavor 字段 (知识点：match 控制流，访问结构体字段)
        // 当风味为 Sweet 时，打印 "Sweet drink"   // 原代码注释
        Flavor::Sweet => println!("Sweet drink"), // 匹配到 Sweet 变体，打印对应信息 (知识点：枚举变体匹配)
        // 当风味为 Fruity 时，打印 "Fruity drink" // 原代码注释
        Flavor::Fruity => println!("Fruity drink"), // 匹配到 Fruity 变体，打印对应信息 (知识点：枚举变体匹配)
        // 当风味为 Spicy 时，打印 "Spicy drink"   // 原代码注释
        Flavor::Spicy => println!("Spicy drink"), // 匹配到 Spicy 变体，打印对应信息 (知识点：穷尽匹配 Exhaustiveness)
    }                                            // match 块结束 (知识点：作用域)
    // 打印饮料的价格信息                           // 原代码注释
    println!("price {}", drink.price);           // 打印 drink 的 price 字段 (知识点：访问结构体字段并打印)
}                                                // print_drink 函数结束，由于传入的 drink 未返回，其内存在此处被释放 (Drop) (知识点：生命周期与内存释放)

impl Drink {                                     // 为 Drink 结构体开启实现块 (知识点：impl 块)
    const PI: f64 = 3.14;                        // 定义 Drink 的关联常量 PI (与 Person::PI 独立互不干扰) (知识点：关联常量，命名空间隔离)

    // 关联函数，用于创建一个新的 Drink 实例           // 原代码注释
    fn new(flavor: Flavor, price: f64) -> Self { // 定义 new 关联函数作为 Drink 的构造器 (知识点：关联函数，Self 别名)
        Self { flavor, price }                   // 实例化 Drink 对象并返回，使用字段名简写语法 (知识点：结构体实例化简写)
    }                                            // new 函数结束 (知识点：作用域)

    // 属性方法，用于打印饮料的信息                    // 原代码注释
    fn buy(&self) {                              // 定义实例方法 buy，接收对自身的不可变引用 &self (知识点：实例方法，&self 借用)
        if self.price > 5.0 {                    // 通过 self 访问当前实例的 price 字段，并与 5.0 进行条件判断 (知识点：if 控制流，比较运算符)
            println!("buy drink");               // 若价格大于 5.0，则执行购买逻辑打印输出 (知识点：if 分支执行)
        } else {                                 // 若价格不大于 5.0 的备用分支 (知识点：else 备用分支)
            println!("no buy drink");            // 打印不购买的提示 (知识点：控制流执行)
        }                                        // if-else 控制流结束 (知识点：作用域)
    }                                            // buy 方法结束 (知识点：作用域)
}                                                // Drink 的 impl 块结束 (知识点：作用域)

// 主函数程序入口                                   // 原代码注释
fn main() {                                      // 声明程序的主入口点 main (知识点：程序入口)
    // 创建一个甜味饮料实例，价格为5.0                 // 原代码注释 (注：实际代码中赋予的价格是 2.0)
    let sweet = Drink {                          // 使用传统的结构体实例化语法，将其绑定到 sweet 变量 (知识点：结构体实例直接构造)
        flavor: Flavor::Sweet,                   // 指定 flavor 字段的值为 Flavor 枚举的 Sweet 变体 (知识点：为枚举字段赋值)
        price: 2.0,                              // 指定 price 字段的值为 2.0 (知识点：为浮点数字段赋值)
    };                                           // 实例化语句结束 (知识点：语法细节)

    let drink = Drink::new(Flavor::Sweet, 1.0);  // 通过调用关联函数（构造器）Drink::new 来实例化对象 (知识点：调用关联函数)
    println!("drink {} ", drink.price);          // 打印通过 new 构造的 drink 实例的 price 属性 (知识点：访问实例字段)

    // 打印甜味饮料的价格                             // 原代码注释
    println!("sweet {}", sweet.price);           // 打印之前直接实例化的 sweet 对象的 price 属性 (知识点：访问实例字段)
    println!("{}", Drink::PI);                   // 直接通过类型名访问 Drink 的关联常量 PI 并打印 (知识点：访问关联常量，不依赖实例)
    sweet.buy();                                 // 调用 sweet 实例的 buy 方法，自动传入 &sweet 引用 (知识点：方法调用与自动借用)

    // 创建一个名为Alice、年龄为30的人的实例            // 原代码注释
    let person = Person::new(String::from("Alice"), 30); // 使用关联函数 new 创建 Person 实例，通过 String::from 分配动态字符串 (知识点：结构体实例化，字符串堆分配)

    // 打印人的名字和年龄                              // 原代码注释
    println!("person.name {} person.age {}", person.name, person.age); // 通过 . 运算符直接访问并打印 person 的两个公共字段 (知识点：实例字段读取)

    let person1 = Person::new(String::from("Bob"), 18); // 再次使用 new 函数创建第二个 Person 实例 person1 (知识点：多实例创建)
    person.print_person(&person1)                // 调用 person 的 print_person 方法，显式传入 person1 的引用作为参数 (知识点：方法调用传递引用参数)
}                                                // main 函数结束，所有局部变量顺次被释放 (知识点：自动内存回收 Drop trait)