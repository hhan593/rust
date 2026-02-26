// ========== Rust 模块可见性（pub）知识点总结 ==========
//
// 【1. 默认私有】
//   Rust 中所有项（模块、函数、结构体、常量等）默认是私有的，
//   只有同一模块内的代码可以访问。
//
// 【2. pub 关键字】
//   加上 pub 后，该项对外部模块可见。
//   但注意：要访问一个深层的项，整条路径上的每一层都必须是 pub 的。
//   例如要从外部调用 hosting::print_name()，需要：
//     - front_of_house 是 pub mod
//     - hosting 是 pub mod
//     - print_name 是 pub fn
//   三者缺一不可。
//
// 【3. 父模块无法访问子模块的私有项】
//   front_of_house 不能直接调用 hosting 中的私有函数 seat_at_table()。
//
// 【4. 子模块可以访问祖先模块的所有项】
//   hosting 内部可以访问 front_of_house 中的私有项（包括 serving 模块）。
//   这是 Rust 的特殊规则：子模块天然可以看到祖先的私有内容。
//
// 【5. 路径引用方式】
//   - crate::xxx    绝对路径，从 crate 根开始（类似 /root/xxx）
//   - super::xxx    相对路径，从父模块开始（类似 ../xxx）
//   - self::xxx     当前模块（通常可省略）
//   - 直接调用       同一模块内的项可以直接用名字调用
//
// 【6. main.rs 与 lib.rs 的关系】
//   它们是两个独立的 crate（bin crate 和 lib crate），
//   main.rs 要使用 lib.rs 的内容，必须通过 crate 名称引用：
//     use restaurant::front_of_house::hosting;
//   其中 "restaurant" 是 Cargo.toml 中 [package] name 的值。
//
// 【7. pub 的更细粒度控制】
//   - pub(crate)    仅当前 crate 内可见
//   - pub(super)    仅父模块可见
//   - pub(in path)  仅指定路径内可见
// ========================================================

// pub mod: 外部（包括 main.rs）可以访问此模块
pub mod front_of_house {

    // pub mod: 外部可以访问此子模块
    pub mod hosting {
        // 私有常量：只有 hosting 模块内部能用
        const NAME: &str = "Alice";

        // pub fn: 外部可以调用
        pub fn add_to_waitlist() {}

        // 私有函数：只有 hosting 模块内部能用，父模块 front_of_house 也不能调用
        fn seat_at_table() {}

        // pub fn: 外部可以调用；内部可以访问同模块的私有常量 NAME
        pub fn print_name() {
            println!("name is {}", NAME);
        }
    }

    // 私有模块：只有 front_of_house 内部能访问，外部完全看不到
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// pub fn: 这是 crate 根级别的公开函数，main.rs 可通过 restaurant::eat() 调用
// 使用 crate:: 绝对路径访问内部模块的公开函数
pub fn eat() {
    crate::front_of_house::hosting::add_to_waitlist();
}
