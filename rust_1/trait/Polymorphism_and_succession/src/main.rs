// === 知识点总结 ===
// 1. 面向对象多态 (Polymorphism)：通过定义通用的 Trait，让不同的类型具备相同的行为接口，这是多态的基础。
// 2. 特征对象与动态分发 (Trait Objects & Dynamic Dispatch)：`&dyn Driver` 是一个特征对象。编译器在编译时不知道具体的类型（是 Car 还是 Suv），而是在程序运行时通过虚表 (vtable) 动态查找并调用具体类型的方法。
// 3. 隐式返回 (Implicit Return)：`road` 函数中最后一行表达式没有分号，因此其计算结果会直接作为整个函数的返回值返回。
// 4. 返回值被丢弃 (Dropped Return Value)：在 `main` 函数中，`road(&Car)` 的调用产生了一个 String 返回值，但没有被绑定到任何变量，因此该返回值在语句结束后会被安全丢弃。
// 5. 单元结构体 (Unit-Like Structs)：没有任何字段的结构体，纯粹用于实现某些行为。

//多态 当前代码段的主题是多态。
trait Driver { // 知识点：定义特征 (Trait)。解释：定义一个通用的 Driver 接口。
    fn drive(&self) -> String; // 知识点：特征方法签名。解释：规定所有实现 Driver 的类型，都必须提供一个返回 String 的 drive 方法。
} // 知识点：作用域结束。解释：Driver 特征定义完毕。

struct Car; // 知识点：定义单元结构体。解释：定义一个代表小汽车的空结构体 Car。
impl Driver for Car { // 知识点：特征实现 (Impl Trait)。解释：将 Driver 特征赋予 Car。
    fn drive(&self) -> String { // 知识点：方法重写。解释：实现 Car 专属的 drive 逻辑。
        println!("Car is driving"); // 知识点：标准输出。解释：打印运行状态。
        String::from("Car is driving") // 知识点：隐式返回、堆分配字符串。解释：创建并直接返回一个 String，不加分号。
    }
}

struct Suv; // 知识点：定义单元结构体。解释：定义一个代表 SUV 的空结构体 Suv。
impl Driver for Suv { // 知识点：特征实现 (Impl Trait)。解释：将 Driver 特征赋予 Suv。
    fn drive(&self) -> String { // 知识点：方法重写。解释：实现 Suv 专属的 drive 逻辑。
        println!("Suv is driveing"); // 知识点：标准输出。解释：打印运行状态（单词拼写保留了原代码的 driveing）。
        String::from("Suv is driveing") // 知识点：隐式返回。解释：创建并返回 SUV 的状态字符串。
    }
}

fn road(vehicle: &dyn Driver) -> String { // 知识点：特征对象传参、显式返回值声明。解释：定义函数，接收任意实现了 Driver 的类型引用，并声明将返回一个 String。
    vehicle.drive() // 知识点：动态分发方法调用、隐式返回。解释：在运行时查表调用传入实例的实际 drive 方法，并将返回的 String 顺延作为 road 函数的返回值（无分号）。
}

fn main() { // 知识点：程序入口。解释：Rust 程序的主线程从这里开始。
    road(&Car); // 知识点：函数调用、取引用。解释：将 Car 实例的引用传入 road 函数。产生的 String 返回值因为没有被接收，随语句结束被自动丢弃。
} // 知识点：作用域结束。解释：main 函数执行完毕。
