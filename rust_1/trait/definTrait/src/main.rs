// === 知识点总结 ===
// 1. 自定义特征 (Custom Traits)：用户可以自由定义 Trait，用于抽象和规范特定的业务行为。
// 2. 特征多重实现 (Multiple Trait Implementations)：Rust 允许为同一个结构体分别实现多个不同的 Trait，使其具备多种能力。
// 3. 内联多重特征约束 (Inline Multiple Trait Bounds)：在泛型声明处，使用 `<T: TraitA + TraitB>` 语法，强制要求传入的类型必须同时具备这些特征。
// 4. where 子句多重约束 (Where Clause Bounds)：当约束条件较长或有多个泛型参数时，使用 `where T: TraitA + TraitB` 语法可以大大提高代码的可读性。

trait Fly { // 知识点：自定义特征。解释：定义一个名为 Fly 的特征，代表具备飞行的能力。
    fn fly(&self); // 知识点：特征方法签名。解释：定义一个需要实现的方法 fly，无默认实现。
} // 知识点：作用域结束。解释：Fly 特征定义结束。

trait Swim { // 知识点：自定义特征。解释：定义一个名为 Swim 的特征，代表具备游泳的能力。
    fn swim(&self); // 知识点：特征方法签名。解释：定义 swim 方法，无默认实现。
} // 知识点：作用域结束。解释：Swim 特征定义结束。

struct Duck { // 知识点：结构体定义。解释：定义一个名为 Duck（鸭子）的结构体。
    name: String, // 知识点：结构体字段。解释：包含一个 name 字段用于存储鸭子的名字。
} // 知识点：作用域结束。解释：结构体定义结束。

impl Fly for Duck { // 知识点：实现特征。解释：为 Duck 结构体实现 Fly 特征。
    fn fly(&self) { // 知识点：实现方法。解释：提供 fly 方法的具体逻辑，使用 &self 读取实例数据。
        println!("{} is flapping its wings!", self.name); // 知识点：标准输出。解释：打印出鸭子飞行的动作。
    } // 知识点：作用域结束。解释：方法结束。
} // 知识点：作用域结束。解释：Fly 的实现结束。

impl Swim for Duck { // 知识点：实现特征。解释：为同一个 Duck 结构体实现 Swim 特征（多重实现）。
    fn swim(&self) { // 知识点：实现方法。解释：提供 swim 方法的具体逻辑。
        println!("{} is paddling in the water!", self.name); // 知识点：标准输出。解释：打印出鸭子游泳的动作。
    } // 知识点：作用域结束。解释：方法结束。
} // 知识点：作用域结束。解释：Swim 的实现结束。

// 场景 1：内联多重约束
fn animal_show<T: Fly + Swim>(animal: &T) { // 知识点：内联多重特征约束。解释：声明泛型 T，使用 `+` 要求 T 必须同时实现 Fly 和 Swim。
    println!("--- Animal Show Starts ---"); // 知识点：标准输出。解释：打印提示信息。
    animal.fly(); // 知识点：多态方法调用。解释：因为约束了 Fly，所以这里可以安全调用 fly()。
    animal.swim(); // 知识点：多态方法调用。解释：因为约束了 Swim，所以这里也可以安全调用 swim()。
} // 知识点：作用域结束。解释：animal_show 函数结束。

// 场景 2：使用 where 子句的多重约束（推荐用于复杂场景）
fn advanced_animal_show<T>(animal: &T) // 知识点：泛型函数声明。解释：仅声明泛型 T，不在此处写约束，使签名更干净。
where // 知识点：where 子句关键字。解释：将泛型约束移到函数签名下方。
    T: Fly + Swim, // 知识点：where 子句多重约束。解释：功能上等同于 `<T: Fly + Swim>`，要求 T 同时具备两种特征。
{ // 知识点：函数体开始。
    println!("--- Advanced Show Starts ---"); // 知识点：标准输出。解释：打印提示信息。
    animal.swim(); // 知识点：方法调用。解释：调用 swim() 方法。
    animal.fly(); // 知识点：方法调用。解释：调用 fly() 方法。
} // 知识点：作用域结束。解释：advanced_animal_show 函数结束。

fn main() { // 知识点：程序入口。解释：主函数开始执行。
    let donald = Duck { // 知识点：结构体实例化。解释：创建 Duck 的一个实例。
        name: String::from("Donald"), // 知识点：字段初始化。解释：赋予鸭子名字。
    }; // 知识点：语句结束。解释：donald 实例初始化完成。

    animal_show(&donald); // 知识点：多约束泛型函数调用。解释：传入 donald 的引用。因为 Duck 同时实现了 Fly 和 Swim，所以编译通过。

    advanced_animal_show(&donald); // 知识点：where子句泛型函数调用。解释：同样传入 donald 的引用，验证 where 子句的效果。
} // 知识点：作用域结束。解释：main 函数结束。