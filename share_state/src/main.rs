use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
    //使用互斥器控制访问
    //互斥器（mutex）是互相排斥（mutual exclusion）的缩写
    // 在使用数据之前，必须获取锁。
    // 使用完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁。

    // Mutex<T> 的 API
    //
    // let m = Mutex::new(5);
    // {
    //     let  mut num = m.lock().unwrap();
    //     *num = 7;
    // } 走出作用域自动释放
    // println!("m = {:?}", m);

    // 给 Mutex<T> 共享访问
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); //所以 Rust 告诉我们，不能将 counter 锁的所有权移动到多个线程中
}
