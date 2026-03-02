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
}
