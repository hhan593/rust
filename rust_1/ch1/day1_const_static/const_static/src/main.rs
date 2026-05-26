// --- 全局作用域 ---
// 真正想全局访问的 const 和 static 应该定义在函数外部（模块级）
static GLOBAL_APP_NAME: &str = "MyRustApp";

fn main() {
    const GLOBAL_YEAR_DAY: i32 = 2020;
    //
    {
        // --- let (变量) ---
        // 1. 作用域：块级作用域，出了 {} 就会失效
        // 2. 内存：在栈上分配，运行时初始化
        // 3. 可变性：默认不可变，加 mut 后可修改
        let month_day = 30;

        // --- 局部的 const ---
        // 1. 作用域：【依然是块级作用域】！出了 {} 名字就失效了。
        // 2. 内存：编译期内联（类似宏替换），不占固定内存地址。
        const INNER_YEAR_DAY: usize = 365;

        println!("内部访问 INNER_YEAR_DAY: {}", INNER_YEAR_DAY); // ✅ 正常打印
        println!("内部访问 month_day: {}", month_day); // ✅ 正常打印

        // 内部也可以访问全局变量
        println!("内部访问全局常量: {}", GLOBAL_YEAR_DAY);
    }

    // --- static (静态变量) 示例 ---
    // 1. 作用域：这取决于它定义在哪里。写在函数里就是局部可见，写在外面就是全局可见。
    // 2. 内存：拥有【固定的内存地址】，整个程序运行期间只有一份实例（生命周期为 'static）。
    // 3. 安全：如果是 static mut，读写必须包裹 unsafe 代码块。
    static LOCAL_STATIC_STR: &str = "LocalStatic";
    println!("访问局部静态变量: {}", LOCAL_STATIC_STR); // ✅ 正常，因为它在 main 函数的作用域内

    println!("外部访问全局静态变量: {}", GLOBAL_APP_NAME); // ✅ 正常，来自外部定义
    println!("外部访问全局常量: {}", GLOBAL_YEAR_DAY); // ✅ 正常，来自外部定义

    // ❌ 下面这两行如果取消注释都会报错，因为它们的作用域局限在上面的 {} 中
    println!("外部访问 INNER_YEAR_DAY: {}", GLOBAL_APP_NAME);
    // println!("{}", month_day);
}