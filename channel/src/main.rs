use std::sync::mpsc; // Multi-producer, single-consumer (多生产者，单消费者)
use std::thread;
use std::time::Duration;

fn main() {
    // 1. 创建信道 (Channel)
    // mpsc::channel() 返回一个元组 (tx, rx)
    // tx (Transmitter): 发送端，用于发送数据。它可以被克隆 (Clone)，从而拥有多个发送者。
    // rx (Receiver): 接收端，用于接收数据。它不能被克隆，只能有一个接收者。
    // 数据在通过信道发送时，所有权会发生转移 (Move)。
    let (tx, rx) = mpsc::channel();

    // 2. 克隆发送端
    // 为了启动第二个生产者线程，我们需要另一个发送端。
    // clone() 方法创建了一个新的发送句柄，它们都指向同一个内部信道缓冲区。
    // 此时：tx 属于主线程（稍后传给线程2），tx1 属于即将创建的线程1。
    let tx1 = tx.clone();

    /*
     【知识点：所有权转移】
    如果取消下面代码的注释，你会看到 Rust 的所有权机制在起作用：
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
         send() 方法获取 val 的所有权并将其放入信道。
         一旦发送完成，当前作用域就不再拥有 val。

         println!("val is {val}"); // 编译错误！val 已被移动 (moved)。
         这保证了内存安全：同一时间只有一个线程能访问这块数据。
    });
    */

    // ------------------------------------------------------------------
    // --- 生产者线程 1 (使用克隆的发送端 tx1) ---
    // ------------------------------------------------------------------
    // 使用 move 关键字：强制将 tx1 的所有权移入新线程。
    // 当这个线程结束时，tx1 会被销毁 (Drop)。
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // 发送数据。如果接收端已断开连接，send 会返回 Err。
            // 这里使用 unwrap() 简单处理，生产环境中建议匹配 Result。
            tx1.send(val).unwrap();

            // 模拟耗时操作，每隔 1 秒发送一条消息
            thread::sleep(Duration::from_secs(1));
        }
        // 循环结束，tx1 离开作用域被销毁。
        // 注意：此时 tx (主线程持有的那个) 仍然存活，所以信道没有关闭。
    });

    // ------------------------------------------------------------------
    // --- 生产者线程 2 (使用原始的发送端 tx) ---
    // ------------------------------------------------------------------
    // 同样使用 move 将 tx 移入此线程。
    // 这是第二个独立的生产者。
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // 循环结束，tx 离开作用域被销毁。
        // 关键点：当 tx 和 tx1 (线程1中的) 都被销毁后，信道内部标记为“无发送者”。
    });
}

// ------------------------------------------------------------------
// --- 消费者 (在主线程中) ---
// ------------------------------------------------------------------
// rx 实现了 Iterator trait。
// 调用 rx.iter() (或者直接在 for 循环中使用 rx) 会创建一个迭代器。
//
// 行为逻辑：
// 1. 阻塞等待：如果信道为空但有
