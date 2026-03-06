pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // 范围检查：如果不符合 [1, 100]，程序直接“罢工”(panic)
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)] // 告诉编译器：这块代码只有执行 `cargo test` 时才编译
mod tests {
    use super::*;

    // 测试 1：验证“异常处理”逻辑
    #[test]
    // #[should_panic] 告诉 Rust：这个测试运行到崩溃才是“成功”的！
    // expected 属性更严谨：它检查崩溃时的错误信息里是否包含这段文字
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200); // 传入 200，预期会触发 panic
    }

    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }

    // 测试 2：使用 Result 编写测试
    // 这种写法的好处是：你可以直接在测试里使用 `?` 操作符，代码更简洁
    #[test]
    #[ignore = "reason"]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(()) // 返回 Ok 代表测试通过
        } else {
            // 返回 Err 代表测试失败，里面的字符串就是失败原因
            Err(String::from("two plus two does not equal four"))
        }
    }

    //如果我们只希望运行被忽略的测试，可以使用 cargo test -- --ignored
    //如果你希望不管是否忽略都要运行全部测试，可以运行 cargo test -- --include-ignored
}
