use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// Box<T>：我演示了两种用法，一种是简单的堆分配（存数字），另一种是解决递归类型（链表）的大小未知问题。
// Rc<T>：展示了引用计数的变化。Rc::clone() 只增加计数，不深拷贝数据。
// RefCell<T>：展示了内部可变性。即使变量是不可变的，也能修改其内部。
// Rc<RefCell<T>>：这是单线程下非常强大的组合，既有多所有权，又能内部修改。
// Arc<Mutex<T>>：这是多线程下的标准配置。Arc 处理跨线程的所有权共享，Mutex 处理数据竞争

// 为了演示 Box 处理递归类型，我们需要定义一个 List 枚举
// 因为它需要被 Box 包装，所以放在外面
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("=== Rust 智能指针综合演示 ===\n");

    // 1. Box<T> 演示
    println!("1. Box<T> 演示 - 堆上分配与递归类型");
    // 1.1 基本用法：在堆上存一个数字
    let boxed_num = Box::new(42);
    println!("解引用 Box: {}", *boxed_num); // * 解引用获取值

    // 1.2 递归类型：构建一个链表 (1 -> 2 -> Nil)
    // 编译器必须知道类型大小，List 是未知大小的，所以必须用 Box 包装
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("链表结构: {:?}\n", list);

    // 2. Rc<T> 演示 (引用计数，单线程共享)
    println!("2. Rc<T> 演示 - 单线程共享所有权");
    let shared_data = Rc::new(vec![1, 2, 3]);
    println!("初始引用计数: {}", Rc::strong_count(&shared_data));

    {
        // 创建一个新的作用域
        let cloned_a = Rc::clone(&shared_data); // 计数+1
        let cloned_b = Rc::clone(&shared_data); // 计数+1
        println!("在作用域内，引用计数: {}", Rc::strong_count(&shared_data));
        // 这里 cloned_a 和 cloned_b 会离开作用域并被丢弃
    }

    // 作用域结束，计数自动减回去
    println!("作用域外，引用计数: {}\n", Rc::strong_count(&shared_data));

    // 3. RefCell<T> 演示 (内部可变性)
    println!("3. RefCell<T> 演示 - 运行时借用检查");
    let internal_data = RefCell::new(String::from("初始数据"));

    // 即使 internal_data 是不可变的，我们也可以修改其内部
    {
        let mut borrowed = internal_data.borrow_mut(); // 获取可变借用
        borrowed.push_str(" -> 已修改");
        // borrow_mut() 返回的 Guard 在这里被释放，锁自动解开
    }

    println!("RefCell 内容: {}\n", internal_data.borrow());

    // 4. 组合技：Rc<RefCell<T>> (单线程下共享且可变)
    println!("4. 组合技 Rc<RefCell<T>> 演示");
    let shared_mutable = Rc::new(RefCell::new(vec![10, 20]));

    // 克隆 Rc，让多个所有者共享这个 RefCell
    let mutator_1 = Rc::clone(&shared_mutable);
    let mutator_2 = Rc::clone(&shared_mutable);

    // 通过任意一个 Rc 都可以修改内部数据
    mutator_1.borrow_mut().push(30);
    mutator_2.borrow_mut().push(40);

    println!("共享可变数据: {:?}\n", shared_mutable.borrow());

    // 5. 多线程演示 (Arc + Mutex)
    // 注意：如果你的环境不支持多线程（如某些 WASM 环境），这部分可能会报错
    // 但在标准 PC 环境下没问题
    println!("5. 多线程演示 Arc<Mutex<T>>");

    // Arc (原子引用计数) 是 Rc 的线程安全版
    // Mutex (互斥锁) 保证线程安全
    let thread_safe_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..3 {
        // 每个线程都需要拥有数据的所有权，所以克隆 Arc
        let counter_per_thread = Arc::clone(&thread_safe_counter);
        let handle = thread::spawn(move || {
            // 获取锁，如果其他线程持有锁，这里会阻塞
            let mut num = counter_per_thread.lock().unwrap();
            *num += 1;
            println!("线程 {} 修改了数据，当前值: {}", i, *num);
        });
        handles.push(handle);
    }

    // 等待所有线程执行完毕
    for handle in handles {
        handle.join().unwrap();
    }

    // 最终结果应该是 3
    println!(
        "\n所有线程结束，最终计数: {}",
        *thread_safe_counter.lock().unwrap()
    );
}
