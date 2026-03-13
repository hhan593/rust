// 派生 PartialEq 和 Debug trait。
// - PartialEq: 允许我们使用 `assert_eq!` 宏来比较两个 Shoe 实例的内容是否相等。
// - Debug: 允许在测试失败时，以开发者友好的格式打印出 Shoe 结构体的内容。
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,     // 鞋码
    style: String, // 鞋子款式
}

// shoes_in_size 函数获取一个鞋子 vector 的所有权和一个鞋码作为参数。
// 它返回一个只包含指定鞋码的鞋子的 vector。
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        // 1. into_iter(): 创建一个获取所有权的迭代器。
        // 这会消耗掉原有的 `shoes` 集合，将其元素的所有权逐个传递给迭代器。
        .into_iter()
        // 2. filter(): 接收一个闭包（匿名函数）作为参数。
        // 闭包 `|s|` 中的参数 `s` 是一双鞋（Shoe）。
        // 这里的闭包捕获了函数环境中的 `shoe_size` 变量，并返回一个布尔值，
        // 只有当闭包返回 true (即 s.size == shoe_size) 时，该元素才会被保留。
        .filter(|s| s.size == shoe_size)
        // 3. collect(): 将迭代器处理后的剩余元素收集起来，
        // 并根据函数的返回值签名（-> Vec<Shoe>）自动转换回一个新的 Vec<Shoe> 集合。
        .collect()
}

// 这是一个测试模块。
// #[cfg(test)] 属性告诉 Rust 编译器：只有在执行 `cargo test` 命令时，才编译和运行这里的代码。
#[cfg(test)]
mod tests {
    // 引入父模块中的所有项（包括 Shoe 结构体和 shoes_in_size 函数），以便在测试中使用。
    use super::*;

    #[test] // 标记这是一个测试函数
    fn filter_by_size() {
        // [准备阶段] 设置测试数据：创建一个包含三双鞋的 vector
        let shoe = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("sandal"),
            },
            Shoe {
                size: 13,
                style: String::from("boot"),
            },
        ];

        // [执行阶段] 调用被测函数：过滤出 10 码的鞋子。
        // 注意：调用后，原有的 `shoe` 变量的所有权已经被转移到了函数内部，不能再被使用了。
        let in_my_size = shoes_in_size(shoe, 10);

        // [断言阶段] 验证结果：检查函数返回的结果是否与预期的（两双 10 码鞋）完全一致。
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("sandal"),
                },
            ]
        )
    }
}
