fn main() {
    // ==========================================
    // 核心知识点 1：进制与数字字面量 (Numeric Literals)
    // ==========================================
    // Rust 默认推断整数类型为 i32，浮点数类型为 f64。
    let a1 = -123;   // 十进制
    let a2 = 0xFF;   // 十六进制 (0x)
    let a3 = 0o13;   // 八进制 (0o)
    let a4 = 0b10;   // 二进制 (0b)

    // 💡 [进阶知识] 字面量后缀与视觉分隔符
    // 可以使用下划线 `_` 提高可读性，并在末尾直接加上类型后缀以明确类型。
    let _salary = 1_000_000_u32;
    let _mask = 0xff_u8;

    // 💡 [进阶知识] 单字节字符字面量 (Byte literals)
    // 使用 b 前缀，类型会被严格推断为 u8，专门用于 ASCII 字节，常用于底层网络/文件处理。
    let _byte_val = b'A'; // 打印的值为 65

    println!("{a1}, {a2}, {a3}, {a4}");


    // ==========================================
    // 核心知识点 2：类型极值与内存占用 (Limits & Size)
    // ==========================================
    println!("i32 max {}", std::i32::MAX);
    println!("i32 min {}", std::i32::MIN);
    // ... 省略其他标准类型的打印，保持代码紧凑 ...

    // 💡 [进阶知识] isize 和 usize 到底有多大？
    // 它们的大小完全取决于你的“计算机架构” (Architecture-dependent)：
    // - 64位系统（如现代 macOS/Win）上占用 8 字节 (64-bit)
    // - 32位系统上占用 4 字节 (32-bit)
    // usize 在实际开发中最核心的用途是作为“数组或集合的索引下标”。
    println!("isize max is {} bytes", std::mem::size_of::<isize>());
    println!("usize max is {} bytes", std::mem::size_of::<usize>());

    // 💡 [进阶知识] 整型溢出保护 (Integer Overflow)
    // 如果变量超出了最大值（例如把 255_u8 加 1）：
    // - Debug 模式 (cargo run)：程序会直接 Panic（崩溃），帮你提前发现 Bug。
    // - Release 模式 (cargo run --release)：执行二进制补码环绕（变成 0），不报错。
    // 想要绝对安全，可以使用安全数学运算：x.checked_add(1) -> 溢出则返回 None


    // ==========================================
    // 核心知识点 3：浮点数 (Floating-Point)
    // ==========================================
    let f1: f32 = 1.1893;
    let f2: f64 = 1.9894;
    // {:.2} 用于格式化输出，会自动进行四舍五入并保留两位小数
    println!("f1: {:.2}, f2: {:.2}", f1, f2);


    // ==========================================
    // 核心知识点 4：布尔值与变量遮蔽 (Shadowing)
    // ==========================================
    let is_ok = true;
    let _not_ok = false;

    // 💡 [进阶知识] 变量遮蔽 (Shadowing) vs 可变变量 (mut)
    // 为什么这里又用 `let` 声明了一次 not_ok 不会报错？
    // 这叫“变量遮蔽”：它创建了一个全新的变量，只是复用了原来的名字。
    // 它的杀手级优势：遮蔽不仅可以赋新值，还可以【改变变量的数据类型】！
    // 如果你用 `mut` 声明变量，你只能修改它的值，绝不可能改变它的数据类型。
    let not_ok = !is_ok;

    println!("is_ok && not_ok {}", is_ok && not_ok); // 逻辑与：均为 true 结果才为 true
    println!("is_ok || not_ok {}", is_ok || not_ok); // 逻辑或：其一为 true 结果就为 true


    // ==========================================
    // 核心知识点 5：字符类型与安全转换 (Char & Casting)
    // ==========================================
    let char_c = 'c';
    let emo_char = '🤭';

    // 💡 [进阶知识] Rust 的 char 固定占用 4 个字节！
    // 和 C/C++ 中的 char（1字节，只能存 ASCII）截然不同，
    // Rust 采用 Unicode 标量值表示字符。这意味一个 char (4 bytes = 32 bits)
    // 可以存放世界上所有的拼音、汉字甚至是 Emoji 表情。
    println!("if you get {} ,you will get {}", char_c, emo_char);

    // 💡 [进阶知识] 显式类型转换 (as 关键字)
    // 重点：Rust 没有任何【隐式类型转换】！（绝不会悄悄把 i32 变 f64，或者 char 变 int）
    // 需要转换类型时，必须使用 `as` 关键字。
    // 下面的代码将 emoji 强转成了它在 Unicode 标准表里的十进制数字编号。
    println!("emo_char 强制转换 usize: {}", emo_char as usize);
    println!("emo_char 强制转换 i32: {}", emo_char as i32);
}