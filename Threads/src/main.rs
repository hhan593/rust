
use std::thread; // 引入线程模块
use std::time::Duration; // 引入时间模块，用于控制睡眠时间

fn main() {
    // thread::spawn 会创建一个新的 OS 线程
    // 它接收一个闭包（closure），闭包里的代码会在新线程中运行
    // 将其返回值存储在 handles 中，其类型是 JoinHandle<T>
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // 使当前（子）线程暂停 1 毫秒，让出 CPU 时间片
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 这部分代码在“主线程”中运行
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        // 使主线程暂停 1 毫秒
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join() 会阻塞当前线程（主线程），直到 handle 所代表的子线程结束
    // 如果不调用 join()，一旦主线程运行结束，程序会直接退出，
    // 不管子线程是否执行完毕（子线程会被强行关掉）。
    // unwrap() 用于处理线程崩溃的情况（如果子线程 panic 了，这里会报错）
    handle.join().unwrap();
}
// ```
//
// ---
//
// ### 几个关键细节：
//
// 1.  **执行顺序的随机性**：虽然我们加了 `sleep`，但主线程和子线程的输出顺序是由操作系统的线程调度器决定的，每次运行的结果可能略有不同。
// 2.  **`JoinHandle` 的作用**：
// * 如果没有 `handle.join().unwrap();`，你可能会发现子线程只打印到 4 或 5 就因为主线程结束而被迫停止了。
// * 通过调用 `join()`，你告诉主线程：“**嘿，等那个伙计干完活你再下班**。”
// 3.  **所有权转移**：如果闭包里需要用到主线程的变量，你通常需要在闭包前加 `move` 关键字（如 `thread::spawn(move || { ... })`），这在处理复杂逻辑时非常常见。
//
