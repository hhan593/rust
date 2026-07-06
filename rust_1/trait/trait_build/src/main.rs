// === 知识点总结 ===
// 1. Trait (特征)：定义一组共享的行为（类似于其他语言中的接口 Interface）。它是一组可以被各个类型实现的方法签名。
// 2. 默认实现 (Default Implementation)：在 Trait 内部可以直接给方法提供方法体。如果实现该 Trait 的类型没有重写该方法，就会自动使用这里的默认实现。
// 3. Struct (结构体)：自定义复合数据类型，包含多个相关的命名字段 (fields)。
// 4. impl Trait for Type (特征实现)：为特定的结构体实现某个 Trait。注意：重写的方法签名必须与 Trait 中定义的严格一致。
// 5. &self (方法接收者)：表示该方法借用调用者实例的不可变引用。带有 `&self` 的叫作方法 (Method)，否则叫关联函数 (Associated Function)。
// 6. impl Trait 语法作参数：`item: &impl Overview` 是一种语法糖，表示函数接受任何实现了 Overview 特征的类型的引用。
// 7. 泛型与特征约束 (Generics & Trait Bounds)：`fn call_overview_generic<T: Overview>(item: &T)` 是上述语法糖的完整等价形式，将泛型 T 约束为必须具备 Overview 能力。

trait Overview {
    // 知识点：定义 Trait。解释：定义一个名为 Overview 的特征，用于约定能够提供概览信息的行为。
    fn overview(&self) -> String {
        // 知识点：方法签名、默认实现。解释：定义需要 `&self` 借用的方法，并提供了一个返回 String 的默认实现。
        "This is an overview".to_string() // 知识点：字符串转换、隐式返回。解释：将字符串字面量转换为 String 类型并返回。
    } // 知识点：作用域结束。解释：overview 默认实现结束。
} // 知识点：作用域结束。解释：Overview 特征定义结束。

trait Summary {
    // 知识点：定义 Trait。解释：定义另一个名为 Summary 的特征。
    fn summarize(&self) -> String {
        // 知识点：方法签名、默认实现。解释：定义 summarize 方法并提供默认实现。
        String::from("(Read more...)") // 知识点：字符串创建、隐式返回。解释：使用 String::from 创建字符串并返回。
    } // 知识点：作用域结束。解释：summarize 默认实现结束。
} // 知识点：作用域结束。解释：Summary 特征定义结束。

struct Course {
    // 知识点：结构体定义。解释：定义 Course 结构体，用于存储课程信息。
    title: String,       // 知识点：结构体字段。解释：名为 title 的 String 类型字段。
    instructor: String,  // 知识点：结构体字段。解释：名为 instructor 的 String 类型字段。
    description: String, // 知识点：结构体字段。解释：名为 description 的 String 类型字段。
} // 知识点：作用域结束。解释：Course 结构体定义结束。

impl Overview for Course {
    // 知识点：实现 Trait。解释：为 Course 结构体单独实现 Overview 特征。
    fn overview(&self) -> String {
        // 知识点：重写特征方法。【修复】：原代码漏掉了 &self。解释：重写 trait 中的 overview 方法，提供属于 Course 的具体逻辑。
        "This is a course overview".to_string() // 知识点：隐式返回。解释：返回特定于 Course 的概览字符串。
    } // 知识点：作用域结束。解释：方法结束。
} // 知识点：作用域结束。解释：Course 的 Overview 实现结束。

struct AbtherCourse {
    // 知识点：结构体定义。解释：定义另一个名为 AbtherCourse 的结构体。
    title: String,       // 知识点：结构体字段。解释：title 字段。
    instructor: String,  // 知识点：结构体字段。解释：instructor 字段。
    description: String, // 知识点：结构体字段。解释：description 字段。
} // 知识点：作用域结束。解释：AbtherCourse 结构体定义结束。

impl Overview for AbtherCourse {
    // 知识点：实现 Trait。解释：为 AbtherCourse 结构体实现 Overview 特征。
    fn overview(&self) -> String {
        // 知识点：重写特征方法。【修复】：原代码漏掉了 &self。解释：重写 overview 方法。
        "This is another course overview".to_string() // 知识点：隐式返回。解释：返回特定于 AbtherCourse 的概览字符串。
    } // 知识点：作用域结束。解释：方法结束。
} // 知识点：作用域结束。解释：AbtherCourse 的 Overview 实现结束。

fn call_overview(item: &impl Overview) {
    // 知识点：impl Trait 语法作参数。解释：定义函数，接收任意实现了 Overview 特征的对象的不可变引用。
    println!("{}", item.overview()); // 知识点：多态、方法调用。解释：调用传入实例的具体 overview 方法并打印。这里会根据具体类型动态/静态分发。
} // 知识点：作用域结束。解释：call_overview 函数结束。

fn call_overview_generic<T: Overview>(item: &T) {
    // 知识点：泛型与 Trait Bounds。解释：这是 `impl Trait` 的完整泛型写法，T 被约束为必须实现 Overview。
    println!("{}", item.overview()); // 知识点：方法调用。解释：调用泛型 T 实例的 overview 方法并打印。
} // 知识点：作用域结束。解释：call_overview_generic 函数结束。

fn main() {
    // 知识点：程序入口。解释：主函数开始执行。
    let c1 = Course {
        // 知识点：结构体实例化。解释：创建 Course 结构体的一个实例，并绑定到变量 c1。
        title: String::from("Rust Programming"), // 知识点：字段初始化。解释：初始化 title 字段。
        instructor: String::from("John Doe"), // 知识点：字段初始化。解释：初始化 instructor 字段。
        description: String::from("This is a course about Rust Programming"), // 知识点：字段初始化。解释：初始化 description 字段。
    }; // 知识点：语句结束。解释：实例 c1 初始化完成。

    let c2 = AbtherCourse {
        // 知识点：结构体实例化。解释：创建 AbtherCourse 结构体的一个实例，并绑定到变量 c2。
        title: String::from("Rust Programming"), // 知识点：字段初始化。解释：初始化 title 字段。
        instructor: String::from("John Doe"), // 知识点：字段初始化。解释：初始化 instructor 字段。
        description: String::from("This is a course about Rust Programming"), // 知识点：字段初始化。解释：初始化 description 字段。
    }; // 知识点：语句结束。解释：实例 c2 初始化完成。

    call_overview(&c1); // 知识点：传递引用。【修复】：原代码缺失了 &，直接传值违背了参数签名的借用。解释：将 c1 的引用传入函数。
    call_overview(&c2); // 知识点：传递引用。【修复】：原代码缺失了 &。解释：将 c2 的引用传入函数。
} // 知识点：作用域结束。解释：main 函数结束，c1 和 c2 离开作用域被销毁。
