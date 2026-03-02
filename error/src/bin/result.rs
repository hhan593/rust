use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
#[derive(Debug)]
fn main() {
    // ==========================================
    // 1. 初始化 Result 变量
    // ==========================================
    // 在赋值的那一刻，value 的状态就已经确定是 Ok 还是 Err 了。
    // 这里我们初始化为 Ok(42)。如果想测试错误流程，可以改为 Err(...)。
    //
    // 使用 &str 而非 String 作为错误类型，因为 Result<i32, &str> 实现了 Copy trait，如果是使用String，则 Result<i32, String> 不实现 Copy，后续使用 value 时会发生所有权转移，导致编译错误。
    // （i32: Copy + &str: Copy → Result<i32, &str>: Copy），
    // 这样 value 可以在多处使用而不会发生所有权转移。
    // 如果错误类型是 String（不实现 Copy），则 map/and_then 等方法会消耗 value，
    // 后续使用需要 .clone() 或 .as_ref()。
    let value: Result<i32, &str> = Ok(42);
    // let value: Result<i32, &str> = Err("出错了"); // 切换测试场景

    // ==========================================
    // 2. map 方法：仅转换成功值 (Ok)
    // ==========================================
    // 逻辑：如果是 Ok(v)，则执行闭包 |v| v * 2，返回 Ok(新值)。
    //       如果是 Err(e)，则不执行闭包，直接原样返回 Err(e)。
    // 结果类型仍然是 Result<i32, &str>。
    println!("map 结果: {:?}", value.map(|v| v * 2));
    // 输出示例 (Ok 场景): map 结果: Ok(84)
    // 输出示例 (Err 场景): map 结果: Err("出错了")

    // ==========================================
    // 3. and_then 方法：链式操作 (扁平化映射)
    // ==========================================
    // 逻辑：如果是 Ok(v)，执行闭包。闭包必须返回一个新的 Result。
    //       如果是 Err(e)，不执行闭包，直接原样返回 Err(e)。
    // 常用于依赖前一步成功结果才能进行的下一步操作（如数据库查询后更新）。
    let result = value.and_then(|v| {
        if v > 0 {
            Ok(v * 2) // 成功，返回新的 Ok
        } else {
            Err("负数不能处理") // 失败，返回新的 Err
        }
    });
    println!("and_then 结果: {:?}", result);

    // ==========================================
    // 4. unwrap_or 方法：提供默认值 (急切求值)
    // ==========================================
    // 逻辑：如果是 Ok(v)，返回 v。如果是 Err，返回提供的默认值。
    // 注意：默认值 0 的类型必须与 Ok 内部的类型 (i32) 一致。
    // 这里需要显式指定泛型 <i32, &str> 帮助编译器推断类型。
    let val_ok: i32 = Ok::<i32, &str>(42).unwrap_or(0);
    println!("val_ok (unwrap_or): {}", val_ok); // 输出: 42

    // 测试 Err 场景
    let val_err_source: Result<i32, &str> = Err("error");
    let val_from_err: i32 = val_err_source.unwrap_or(0);
    println!("val_from_err (unwrap_or): {}", val_from_err); // 输出: 0

    // ==========================================
    // 5. unwrap_or_else 方法：惰性计算默认值
    // ==========================================
    // 逻辑：如果是 Ok(v)，返回 v。如果是 Err，**才执行**闭包来计算默认值。
    // 优势：如果闭包内有耗时操作（如读文件、随机数），在 Ok 情况下不会执行，节省性能。
    // 闭包参数 |_| 中的 _ 代表接收到的错误值 (这里我们不需要用到它，所以用 _ 忽略)。用不到的参数可以用 _ 占位，表示我们不关心这个参数。
    let val_lazy: i32 = Err::<i32, &str>("error").unwrap_or_else(|_err| {
        // println!("发生错误，正在计算默认值..."); // 只有出错时才会打印这行
        120
    });
    println!("val_lazy (unwrap_or_else): {}", val_lazy); // 输出: 120

    // ==========================================
    // 6. unwrap_or_default 方法：使用类型的默认值
    // ==========================================
    // 逻辑：如果是 Ok(v)，返回 v。如果是 Err，返回 T::default()。
    // 对于 i32，default() 就是 0；对于 String，就是 ""；对于 Vec，就是 []。
    let val_default: i32 = Err::<i32, &str>("error").unwrap_or_default();
    println!("val_default (unwrap_or_default): {}", val_default); // 输出: 0

    // ==========================================
    // 7. is_ok / is_err 方法：布尔判断 判断是不是成功或者失败
    // ==========================================
    // 逻辑：简单判断当前状态，返回 bool。
    // 常用于 if 条件判断中。
    assert!(Ok::<i32, &str>(42).is_ok()); // 断言成功：确实是 Ok
    assert!(Err::<i32, &str>("err").is_err()); // 断言成功：确实是 Err

    // 实际使用中常这样写：
    if value.is_ok() {
        println!("程序运行成功！");
    } else {
        println!("程序出错了！");
    }

    // 补充说明：因为 value 是 Copy 类型（Result<i32, &str>），
    // 上面对 value 调用 map/and_then/is_ok 等方法后，value 仍然有效。
    let another_value: Result<i32, &str> = Ok(100);
    println!("另一个值的结果: {:?}", another_value.is_ok()); // 输出: true

    //     方法名称,关注点,大白话解释,是否用到前一步内部的值？

    let ok: Result<i32, &str> = Ok(42);
    let err: Result<i32, &str> = Err("error");
    // and,连续成功,第一步成了，直接看第二步的结果。,否（不关心第一步拿到了啥，只要是 Ok 就行）
    println!("and: {:?}", ok.and(Ok(100))); // Ok(100);

    // and_then,连续加工,第一步成了，拿着第一步的东西去做第二步。,是（用到前一个 Ok 里的值）
    ok.and_then(|v| {
        if v > 0 {
            Ok(v * 2)
        } else {
            Err("必须为正数")
        }
    }); // Ok(84);

    // or,失败替补,第一步砸了，直接用备选方案顶替。,否（不关心第一步是怎么砸的）,// or: 类似 || —— self 为 Err 时返回 other，否则返回 self 的 Ok 值。
    ok.or(Err("没有这个文件")); // Ok(42)，因为 ok 是 Ok，不会用到 Err 备选方案;
    // err.or(Ok(42)); // Ok(42)，因为 err 是 Err，会用到 Ok 备选方案;

    // or_else,失败抢救,第一步砸了，分析砸掉的原因，尝试抢救。是（用到前一个 Err 里的错误信息）
    err.or_else(|e| {
        if e == "recoverable" {
            Ok(0)
        } else {
            Err("still broken")
        }
    }); // Err("still broken")

    println!("ok 的值: {:?}", ok.and(Err::<i32, &str>("other error")));

    //     | 方法              | 适用场景                           |
    // | --------------- | ------------------------------ |
    // | `unwrap()`      | 快速原型、示例代码、确信不会失败的情况            |
    // | `expect("msg")` | 生产代码中需要 panic 的地方（提供有意义的上下文信息） |

    //如果文件不存在，会panic 并显示默认错误信息，提示我们文件无法打开。
    let result = File::open("hello.txt").unwrap();

    //如果文件不存在，会panic，并显示自定义错误信息,这个方法和 unwrap 类似，但允许我们提供一个自定义的错误消息，帮助我们更快地定位问题。

    let result = File::open("hello.txt").expect("无法打开 hello.txt 文件");

    // | 方法                    | Ok/Some 时 | Err/None 时   | 适用场景           |
    // | --------------------- | --------- | ------------ | -------------- |
    // | `unwrap()`            | 返回值       | panic（默认信息）  | 原型代码           |
    // | `expect("msg")`       | 返回值       | panic（自定义信息） | 确信不会失败         |
    // | `unwrap_or(default)`  | 返回值       | 返回 default   | 有合理默认值         |
    // | `unwrap_or_else(f)`   | 返回值       | 调用 f()       | 默认值需要计算        |
    // | `unwrap_or_default()` | 返回值       | T::default() | 类型有 Default 实现 |
    // | `unwrap_err()`        | panic     | 返回错误值        | 测试代码中断言错误      |
}
