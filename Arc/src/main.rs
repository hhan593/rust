//`Arc<T>`（Atomic Reference Counting）是 `Rc<T>` 的线程安全版本，使用原子操作来维护引用计数，因此可以安全地在多个线程间共享。

//搭配 Mutex 实现线程间共享可变数据
use std::sync::{Arc, Mutex}; // 引入 Arc (多线程共享指针) 和 Mutex (互斥锁)
use std::thread; // 引入线程模块

fn main() {
    // 1. 创建一个被 Mutex 保护的整数 0。
    // 2. 用 Arc 将其包装，以便它可以安全地在多个线程之间拥有“所有权”。
    let counter = Arc::new(Mutex::new(0));

    // 用于存储线程句柄，以便后续等待它们执行完毕
    let mut handles = vec![];

    for _ in 0..10 {
        // 克隆 Arc 指针。这不会克隆内部的数据，只是增加引用计数。
        // 每个线程都会获得一个指向相同内存地址的“克隆指针”。
        let counter = Arc::clone(&counter);

        // 创建一个新线程
        // 'move' 关键字将克隆后的 counter 的所有权移动到线程闭包内
        let handle = thread::spawn(move || {
            // 获取互斥锁。如果锁被占用，线程会在此阻塞直到获取成功。
            // unwrap() 用于处理可能出现的“锁中毒”情况（即另一个线程在持有锁时崩溃了）。
            let mut num = counter.lock().unwrap();

            // 解引用并修改内部数值
            *num += 1;

            // 当 num 离开作用域时，MutexGuard 会自动析构，从而释放锁
        });

        // 将线程句柄存入数组
        handles.push(handle);
    }

    // 遍历所有句柄，调用 join() 确保主线程等待所有子线程执行结束
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印最终结果，此时引用计数已经回落，再次获取锁并打印值
    println!("Result: {}", *counter.lock().unwrap()); // 预期输出: 10
}
// 关键概念解析
// Arc<T> (Atomic Reference Counted):
// 普通的 Rc<T> 只能在单线程中使用。Arc 是线程安全的引用计数，它允许数据有多个“所有者”，当最后一个 Arc 被销毁时，数据才会被释放。
//
// Mutex<T> (Mutual Exclusion):
// 由于 Arc 只提供共享读取权限，我们需要 Mutex 来提供内部可变性。它确保同一时间只有一个线程可以访问/修改内部的数字。
//
// lock().unwrap():
// 在 Rust 中，获取锁可能会失败（如果之前持有锁的线程 panic 了，锁会进入 "poisoned" 状态）。unwrap() 是开发中最常见的处理方式，表示如果锁坏了就直接让当前线程也崩溃。