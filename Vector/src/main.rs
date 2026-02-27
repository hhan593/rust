// ============================================================================
// Rust Vector (Vec<T>) 全面知识手册
// ============================================================================
// 参考来源:
//   - https://doc.rust-lang.org/std/vec/struct.Vec.html
//   - https://doc.rust-lang.org/book/ch08-01-vectors.html
//   - https://doc.rust-lang.org/rust-by-example/std/vec.html
//   - https://google.github.io/comprehensive-rust/std-types/vec.html
// ============================================================================

fn main() {
    // ========================================================================
    // 1. Vector 基础概念
    // ========================================================================
    // Vec<T> 是 Rust 标准库中最常用的集合类型之一
    // - 在堆（heap）上分配内存，存储同一类型的多个值
    // - 内部结构: (指针 pointer, 容量 capacity, 长度 length) 三元组
    // - 支持动态增长和缩小
    // - 索引访问 O(1)，尾部 push/pop 均摊 O(1)

    // ========================================================================
    // 2. 创建 Vector
    // ========================================================================

    // 方式一: Vec::new() — 创建空 vector（需要类型注解或后续推断）
    let _v1: Vec<i32> = Vec::new();

    // 方式二: vec! 宏 — 用初始值创建（Rust 自动推断类型）
    let _v2 = vec![1, 2, 3]; // Vec<i32>
    let _v3 = vec![0; 5]; // 创建包含 5 个 0 的 vector: [0, 0, 0, 0, 0]
    let _v4 = vec!["hello".to_string(), "world".to_string()]; // Vec<String>

    // 方式三: Vec::with_capacity() — 预分配容量（避免频繁重新分配）
    let mut v5 = Vec::with_capacity(10);
    v5.push(1); // 在容量范围内 push 不会触发重新分配
    println!("v5 长度: {}, 容量: {}", v5.len(), v5.capacity()); // 长度: 1, 容量: 10

    // 方式四: 从迭代器收集
    let _v6: Vec<i32> = (0..10).collect(); // [0, 1, 2, ..., 9]
    let _v7: Vec<i32> = (0..10).filter(|x| x % 2 == 0).collect(); // [0, 2, 4, 6, 8]

    // 方式五: 从数组/切片转换
    let arr = [1, 2, 3];
    let _v8 = arr.to_vec(); // 从数组转换
    let _v9 = Vec::from([4, 5, 6]); // From trait 转换

    // ========================================================================
    // 3. 访问元素
    // ========================================================================

    let values = vec![10, 20, 30, 40, 50];

    // 方式一: 索引访问（越界会 panic!）
    // 适用场景: 你 100% 确定索引在范围内
    let third = &values[2];
    println!("第三个元素: {}", third); // 30

    // 方式二: .get() 方法（越界返回 None，不会 panic）
    // 适用场景: 索引可能越界（如用户输入）
    match values.get(2) {
        Some(val) => println!("第三个元素: {}", val), // 30
        None => println!("索引越界!"),
    }
    // get 也支持范围
    let slice = values.get(1..3); // Some(&[20, 30])
    println!("切片: {:?}", slice);

    // 获取首尾元素
    println!("第一个: {:?}", values.first()); // Some(10)
    println!("最后一个: {:?}", values.last()); // Some(50)

    //  ┌────────────────────────────────────────────────────────────┐
    //  │ 访问方式  │  语法         │  越界行为      │  适用场景     │
    //  │ 下标索引  │  &v[index]    │  panic 崩溃    │  确保不越界时 │
    //  │ get 方法  │  v.get(index) │  返回 None     │  索引可能越界 │
    //  └────────────────────────────────────────────────────────────┘

    // ========================================================================
    // 4. 修改 Vector
    // ========================================================================

    let mut v = vec![1, 2, 3];

    // --- 添加元素 ---
    v.push(4); // 尾部添加: [1, 2, 3, 4]
    v.insert(1, 10); // 在索引 1 处插入: [1, 10, 2, 3, 4] insert(index, value) --> (索引, 值)
    // 注意: insert 会导致后面的元素全部移动，O(n) 复杂度

    // --- 删除元素,注意返回类型是一个Option<T> ---
    let last = v.pop(); // 移除并返回最后一个: Some(4), v=[1, 10, 2, 3]
    println!("pop: {:?}", last);

    let removed = v.remove(1); // 移除索引 1 的元素: 10, v=[1, 2, 3]
    println!("remove: {}", removed);

    // swap_remove: O(1) 删除，用最后一个元素替换被删除的位置（不保序）
    let mut v_swap = vec![10, 20, 30, 40];
    let removed = v_swap.swap_remove(1); // 移除 20，用 40 替换: [10, 40, 30]
    println!("swap_remove: {}, 结果: {:?}", removed, v_swap);

    // --- 批量操作 ---
    let mut v = vec![1, 2, 3, 4, 5];

    // retain: 只保留满足条件的元素
    v.retain(|&x| x % 2 != 0); // 保留奇数: [1, 3, 5]
    println!("retain 后: {:?}", v);

    // truncate: 截断到指定长度，从索引0开始保留前 n 个元素，丢弃后面多余的元素
    let mut v = vec![1, 2, 3, 4, 5];
    v.truncate(3); // [1, 2, 3]
    println!("truncate 后: {:?}", v);

    // clear: 清空所有元素（保留已分配的容量）
    let cap_before = v.capacity();
    v.clear();
    println!(
        "clear 后长度: {}, 容量不变: {}",
        v.len(),
        v.capacity() == cap_before
    );

    // resize: 调整大小，不足的部分用指定值填充
    let mut v = vec![1, 2, 3];
    v.resize(5, 0); // [1, 2, 3, 0, 0]
    v.resize(2, 0); // [1, 2] — 超出部分被丢弃
    println!("resize 后: {:?}", v);

    // fill: 用同一个值填充整个 vector
    let mut v = vec![0; 5];
    v.fill(42); // [42, 42, 42, 42, 42]
    println!("fill 后: {:?}", v);

    // append: 将另一个 vector 的所有元素移入（另一个变空）
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];
    v1.append(&mut v2);
    println!("append 后 v1: {:?}, v2: {:?}", v1, v2); // v1=[1,2,3,4,5,6], v2=[]

    // extend_from_slice: 从切片扩展
    let mut v = vec![1, 2];
    v.extend_from_slice(&[3, 4, 5]);
    println!("extend_from_slice 后: {:?}", v); // [1, 2, 3, 4, 5]

    // splice: 替换一个范围内的元素
    let mut v = vec![1, 2, 3, 4, 5];
    let old: Vec<_> = v.splice(1..3, vec![20, 30, 40]).collect();
    println!("splice: 旧值 {:?}, 新 vector {:?}", old, v); // 旧[2,3], 新[1,20,30,40,4,5]

    // split_off: 从指定位置分割成两个 vector
    let mut v = vec![1, 2, 3, 4, 5];
    let v2 = v.split_off(3);
    println!("split_off: v={:?}, v2={:?}", v, v2); // v=[1,2,3], v2=[4,5]

    // drain: 移除并迭代一个范围
    let mut v = vec![1, 2, 3, 4, 5];
    let drained: Vec<_> = v.drain(1..3).collect();
    println!("drain: 取出 {:?}, 剩余 {:?}", drained, v); // 取出[2,3], 剩余[1,4,5]

    // ========================================================================
    // 5. 遍历 Vector
    // ========================================================================

    let mut data = vec![100, 200, 300];

    //  ┌──────────────────────────────────────────────────────────────────────┐
    //  │ 语法              │ i 的类型       │ 所有权        │ 循环后可用?    │
    //  │ for i in v        │ T (值)         │ 转移 (Move)   │ ❌ 不可用      │
    //  │ for i in &v       │ &T (不可变引用) │ 借用 (Borrow) │ ✅ 依然可用    │
    //  │ for i in &mut v   │ &mut T (可变)   │ 借用 (Borrow) │ ✅ 依然可用    │
    //  └──────────────────────────────────────────────────────────────────────┘

    // 不可变遍历
    for val in &data {
        println!("值: {}", val);
    }

    // 可变遍历（修改每个元素）
    for val in &mut data {
        *val += 50; // 使用 * 解引用来修改值
    }
    println!("修改后: {:?}", data); // [150, 250, 350]

    // 带索引遍历
    for (index, val) in data.iter().enumerate() {
        println!("索引 {}: 值 {}", index, val);
    }

    // 消费遍历（vector 之后不可用） 因为把 vector 的所有权转移到循环变量了，循环结束之后就不能再使用 vector 了
    let data = vec![1, 2, 3];
    for val in data {
        println!("消费: {}", val);
    }
    // println!("{:?}", data); // 编译错误! data 已被 move

    // 迭代器方法链
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<i32> = nums
        .iter()
        .filter(|&&x| x % 2 == 0) // 过滤偶数
        .map(|&x| x * x) // 平方
        .collect();
    println!("迭代器链: {:?}", result); // [4, 16, 36, 64, 100]

    // ========================================================================
    // 6. 排序与查找
    // ========================================================================

    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // sort: 稳定排序（不会改变相等元素的相对顺序）
    v.sort();
    println!("sort: {:?}", v); // [1, 1, 2, 3, 4, 5, 6, 9]

    // sort_unstable: 不稳定排序（通常更快）
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    v.sort_unstable();
    println!("sort_unstable: {:?}", v);

    // 自定义排序: sort_by
    let mut v = vec![3, 1, 4, 1, 5];
    v.sort_by(|a, b| b.cmp(a)); // 降序
    println!("降序: {:?}", v); // [5, 4, 3, 1, 1]

    // sort_by_key: 按某个键排序
    let mut words = vec!["banana", "apple", "cherry"];
    words.sort_by_key(|w| w.len());
    println!("按长度排序: {:?}", words); // ["apple", "banana", "cherry"]

    // 浮点数排序（f64 不实现 Ord，需要 partial_cmp）
    let mut floats = vec![3.1, 1.2, 4.5, 1.1];
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("浮点排序: {:?}", floats);

    // reverse: 反转
    let mut v = vec![1, 2, 3];
    v.reverse();
    println!("反转: {:?}", v); // [3, 2, 1]

    // 查找
    let v = vec![1, 2, 3, 4, 5];
    println!("contains 3: {}", v.contains(&3)); // true
    println!("starts_with [1,2]: {}", v.starts_with(&[1, 2])); // true
    println!("ends_with [4,5]: {}", v.ends_with(&[4, 5])); // true

    // binary_search: 二分查找（需要已排序）
    let v = vec![1, 3, 5, 7, 9];
    match v.binary_search(&5) {
        Ok(index) => println!("找到 5，索引: {}", index), // 索引: 2
        Err(index) => println!("未找到，应插入位置: {}", index),
    }

    // iter().position(): 找到第一个满足条件的元素索引
    let v = vec![10, 20, 30, 40];
    let pos = v.iter().position(|&x| x == 30);
    println!("30 的位置: {:?}", pos); // Some(2)

    // iter().find(): 找到第一个满足条件的元素
    let found = v.iter().find(|&&x| x > 25);
    println!("第一个大于25的: {:?}", found); // Some(30)

    // ========================================================================
    // 7. 去重
    // ========================================================================

    // dedup: 移除**连续**重复元素（需要先排序才能去除所有重复）
    let mut v = vec![1, 1, 2, 3, 3, 3, 4, 4, 5];
    v.dedup();
    println!("dedup: {:?}", v); // [1, 2, 3, 4, 5]

    // 完全去重的正确方式: 先排序再 dedup
    let mut v = vec![3, 1, 2, 1, 3, 2, 4];
    v.sort();
    v.dedup();
    println!("排序+dedup: {:?}", v); // [1, 2, 3, 4]

    // dedup_by_key: 按键去重
    let mut v = vec!["foo", "bar", "BAR", "baz", "BAZ"];
    v.dedup_by_key(|s| s.to_lowercase());
    println!("dedup_by_key: {:?}", v); // ["foo", "bar", "baz"]

    // ========================================================================
    // 8. 切片与分块
    // ========================================================================
    // Vec<T> 实现了 Deref<Target = [T]>，因此可以调用所有切片方法

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // 切片（borrowing a section）
    let slice = &v[2..5]; // [3, 4, 5]
    println!("切片: {:?}", slice);

    // chunks: 按固定大小分块
    for chunk in v.chunks(3) {
        println!("chunk: {:?}", chunk);
    }
    // [1, 2, 3], [4, 5, 6], [7, 8, 9]

    // windows: 滑动窗口
    for window in v.windows(3) {
        println!("window: {:?}", window);
    }
    // [1,2,3], [2,3,4], [3,4,5], ... [7,8,9]

    // split_at: 在指定位置分割（返回两个切片的引用）
    let (left, right) = v.split_at(4);
    println!("left: {:?}, right: {:?}", left, right); // [1,2,3,4], [5,6,7,8,9]

    // ========================================================================
    // 9. 容量管理
    // ========================================================================

    let mut v: Vec<i32> = Vec::new();
    println!("初始 — 长度: {}, 容量: {}", v.len(), v.capacity()); // 0, 0

    v.push(1);
    println!("push 1 次 — 长度: {}, 容量: {}", v.len(), v.capacity()); // 1, 4（通常）

    // reserve: 确保至少还能容纳 additional 个元素
    v.reserve(100);
    println!("reserve(100) — 容量: {}", v.capacity()); // >= 101

    // shrink_to_fit: 收缩容量到当前长度
    v.shrink_to_fit();
    println!("shrink_to_fit — 长度: {}, 容量: {}", v.len(), v.capacity()); // 1, 1

    // is_empty: 检查是否为空
    println!("is_empty: {}", v.is_empty()); // false

    //  ┌──────────────────────────────────────────────────────────────────────────┐
    //  │  方法              │  作用                                               │
    //  │  len()             │  返回当前元素数量                                   │
    //  │  capacity()        │  返回当前已分配的容量                               │
    //  │  is_empty()        │  是否为空                                           │
    //  │  reserve(n)        │  确保至少还能再放 n 个元素（可能多分配）            │
    //  │  reserve_exact(n)  │  尽量精确地多分配 n 个元素                          │
    //  │  shrink_to_fit()   │  收缩容量到当前长度                                 │
    //  │  shrink_to(n)      │  收缩到不低于 n 的容量                              │
    //  │  with_capacity(n)  │  创建时预分配 n 个元素的容量                        │
    //  └──────────────────────────────────────────────────────────────────────────┘

    // ========================================================================
    // 10. 所有权与借用规则
    // ========================================================================

    // ❌ 同时存在不可变引用和可变操作 — 编译错误!
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];       // 不可变借用
    // v.push(6);                // 可变借用 — ERROR!
    // println!("{}", first);    // 不可变借用仍在使用

    // 原因: push 可能触发重新分配内存，导致之前的引用悬空（dangling）
    // Rust 的借用检查器在编译时就阻止了这种不安全行为

    // ✅ 正确方式: 确保借用不重叠
    let mut v = vec![1, 2, 3, 4, 5];
    let first = v[0]; // 拷贝值（i32 实现了 Copy），而非借用引用
    v.push(6);
    println!("first: {}, v: {:?}", first, v);

    // ✅ 正确方式: 引用在可变操作之前结束生命周期
    let mut v = vec![1, 2, 3, 4, 5];
    {
        let first = &v[0];
        println!("first: {}", first);
    } // first 的借用在此结束
    v.push(6); // 现在可以可变借用了

    // ========================================================================
    // 11. 用 Enum 在 Vector 中存储多种类型
    // ========================================================================

    // Vec 只能存储同一类型，但 Enum 的不同变体算同一类型!
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(3.14),
    ];

    // 用 match 处理不同变体
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("整数: {}", i),
            SpreadsheetCell::Float(f) => println!("浮点: {}", f),
            SpreadsheetCell::Text(s) => println!("文本: {}", s),
        }
    }

    // ========================================================================
    // 12. 常用转换
    // ========================================================================

    // Vec -> 切片
    let v = vec![1, 2, 3];
    let _s: &[i32] = &v; // 自动 Deref
    let s2 = v.as_slice(); // 显式转换
    println!("切片: {:?}", s2);

    // Vec -> String (针对 Vec<u8>)
    let bytes = vec![72, 101, 108, 108, 111]; // "Hello"
    let s = String::from_utf8(bytes).unwrap();
    println!("bytes -> String: {}", s);

    // String -> Vec<u8>
    let s = String::from("Hello");
    let bytes: Vec<u8> = s.into_bytes();
    println!("String -> bytes: {:?}", bytes);

    // Vec -> Box<[T]>（丢弃多余容量）
    let v = vec![1, 2, 3];
    let boxed: Box<[i32]> = v.into_boxed_slice();
    println!("boxed slice: {:?}", boxed);

    // 数组 -> Vec
    let arr = [1, 2, 3];
    let v = Vec::from(arr);
    println!("from array: {:?}", v);

    // Vec -> 数组（长度必须匹配）
    let v = vec![1, 2, 3];
    let arr: [i32; 3] = v.try_into().unwrap();
    println!("to array: {:?}", arr);

    // 扁平化嵌套 Vec
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("扁平化: {:?}", flat); // [1, 2, 3, 4, 5, 6]

    // ========================================================================
    // 13. 实用模式与技巧
    // ========================================================================

    // --- 用 map 进行变换 ---
    let names = vec!["alice", "bob", "charlie"];
    let upper: Vec<String> = names.iter().map(|n| n.to_uppercase()).collect();
    println!("大写: {:?}", upper);

    // --- fold / reduce 聚合 ---
    let nums = vec![1, 2, 3, 4, 5];
    let sum: i32 = nums.iter().sum();
    let product: i32 = nums.iter().product();
    println!("总和: {}, 乘积: {}", sum, product); // 15, 120

    let max = nums.iter().max();
    let min = nums.iter().min();
    println!("最大: {:?}, 最小: {:?}", max, min); // Some(5), Some(1)

    // --- zip: 配对两个 vector ---
    let keys = vec!["a", "b", "c"];
    let vals = vec![1, 2, 3];
    let pairs: Vec<(&&str, &i32)> = keys.iter().zip(vals.iter()).collect();
    println!("zip: {:?}", pairs); // [("a", 1), ("b", 2), ("c", 3)]

    // --- 分区: partition ---
    let nums = vec![1, 2, 3, 4, 5, 6];
    let (evens, odds): (Vec<&i32>, Vec<&i32>) = nums.iter().partition(|&&x| x % 2 == 0);
    println!("偶数: {:?}, 奇数: {:?}", evens, odds);

    // --- 去除并收集: 使用 drain + filter ---
    let mut v = vec![1, 2, 3, 4, 5];
    let removed: Vec<i32> = v.drain(..).filter(|x| *x > 3).collect();
    println!("drain+filter: {:?}", removed); // [4, 5]

    // --- 安全交换两个元素 ---
    let mut v = vec![10, 20, 30, 40];
    v.swap(0, 3);
    println!("swap 后: {:?}", v); // [40, 20, 30, 10]

    // --- 用 iter().any() / all() 检查条件 ---
    let v = vec![2, 4, 6, 8];
    println!("全部是偶数: {}", v.iter().all(|&x| x % 2 == 0)); // true
    println!("存在大于5的: {}", v.iter().any(|&x| x > 5)); // true

    // --- copy_from_slice: 高效拷贝（长度必须相等）---
    let src = vec![1, 2, 3];
    let mut dst = vec![0; 3];
    dst.copy_from_slice(&src);
    println!("copy_from_slice: {:?}", dst); // [1, 2, 3]

    // --- rotate_left / rotate_right: 旋转元素 ---
    let mut v = vec![1, 2, 3, 4, 5];
    v.rotate_left(2); // [3, 4, 5, 1, 2]
    println!("rotate_left(2): {:?}", v);

    // ========================================================================
    // 14. Vector 与 Trait 对象 (dyn Trait) — 存储不同类型
    // ========================================================================

    // 当编译时不知道所有可能的类型时，使用 trait 对象
    trait Drawable {
        fn draw(&self);
    }

    struct Circle;
    struct Square;

    impl Drawable for Circle {
        fn draw(&self) {
            println!("画圆形 ●");
        }
    }
    impl Drawable for Square {
        fn draw(&self) {
            println!("画方形 ■");
        }
    }

    let shapes: Vec<Box<dyn Drawable>> = vec![Box::new(Circle), Box::new(Square)];
    for shape in &shapes {
        shape.draw();
    }

    println!("\n=== Vector 知识手册运行完毕! ===");
}

// ============================================================================
// 15. 内存布局图解
// ============================================================================
//
//  Stack (栈)                     Heap (堆)
//  ┌───────────────┐              ┌───┬───┬───┬───┬───┬───┬───┬───┐
//  │ ptr ──────────┼─────────────▶│ 1 │ 2 │ 3 │ 4 │ 5 │   │   │   │
//  │ len = 5       │              └───┴───┴───┴───┴───┴───┴───┴───┘
//  │ capacity = 8  │                ▲                   ▲           ▲
//  └───────────────┘                │ 已初始化 (len=5)  │  未初始化  │
//                                   └───────────────────┘           │
//                                       capacity = 8 ──────────────┘
//
// - ptr: 指向堆上分配的内存的指针
// - len: 当前存储的元素数量（只有 len 个元素是已初始化的）
// - capacity: 已分配的总容量（无需重新分配就能存储的最大元素数）
// - 当 len == capacity 时，下一次 push 会触发重新分配（通常 capacity 翻倍）

// ============================================================================
// 16. 生命周期与 Vector
// ============================================================================
//
// Vector 的引用遵循 Rust 的借用规则:
//
//   let mut v = vec![1, 2, 3];
//   let r = &v[0];      // 不可变借用整个 vector
//   v.push(4);        // ❌ 不能可变借用，因为 r 还在使用
//   println!("{}", r);   // r 的最后一次使用
//   v.push(4);           // ✅ 现在可以了（Non-Lexical Lifetimes, NLL）
//
// 为什么 push 时不能有不可变引用?
//   因为 push 可能触发内存重新分配，旧的指针会失效，
//   如果此时还有引用指向旧内存，就会产生悬垂引用（dangling reference）。
//   Rust 的借用检查器在编译时阻止了这种情况。

// ============================================================================
// 17. Vec 与其他集合类型的选择
// ============================================================================
//
//  ┌─────────────────┬────────────────────────────────────────────────────────┐
//  │  类型            │  适用场景                                             │
//  ├─────────────────┼────────────────────────────────────────────────────────┤
//  │  Vec<T>          │  通用动态数组，最常用                                 │
//  │  VecDeque<T>     │  需要高效的头部和尾部操作（双端队列）                 │
//  │  LinkedList<T>   │  需要频繁在中间插入/删除（较少使用）                  │
//  │  HashMap<K,V>    │  键值对查找                                           │
//  │  HashSet<T>      │  不重复元素集合                                       │
//  │  BTreeMap<K,V>   │  有序键值对                                           │
//  │  BTreeSet<T>     │  有序不重复集合                                       │
//  │  BinaryHeap<T>   │  优先队列（总是能快速取出最大值）                     │
//  └─────────────────┴────────────────────────────────────────────────────────┘

// ============================================================================
// 18. 常见陷阱与最佳实践
// ============================================================================
//
// ❌ 陷阱 1: 在持有引用时修改 vector
//    let mut v = vec![1, 2, 3];
//    let first = &v[0];
//    v.push(4);  // 编译错误!
//
// ❌ 陷阱 2: 索引越界
//    let v = vec![1, 2, 3];
//    let x = v[10];  // panic!
//    --> 使用 v.get(10) 代替,此时会返回    None 而非 panic!
//
// ❌ 陷阱 3: 在 for 循环中修改 vector 的结构
//     不能在遍历中 push/remove
//    --> 使用 retain、drain 或先收集索引再修改
//
// ❌ 陷阱 4: 忘记 mut
//    let v = Vec::new();
//    v.push(1);  // 编译错误!
//    --> let mut v = Vec::new();
//
// ✅ 最佳实践 1: 函数参数用 &[T] 而非 &Vec<T>
//    fn process(data: &[i32]) { ... }  // 更通用
//
// ✅ 最佳实践 2: 已知大小时预分配容量
//    let mut v = Vec::with_capacity(1000);
//
// ✅ 最佳实践 3: 不关心顺序时用 swap_remove 代替 remove
//    v.swap_remove(idx);  // O(1) vs O(n)
//
// ✅ 最佳实践 4: 使用迭代器链代替手动循环
//    let result: Vec<_> = v.iter().filter(...).map(...).collect();
//
// ✅ 最佳实践 5: 排序时优先考虑 sort_unstable
//    v.sort_unstable();  // 通常更快

// ============================================================================
// 19. 完整方法速查表
// ============================================================================
//
//  ┌─ 创建 ───────────────────────────────────────────────────────────────┐
//  │  Vec::new()                  创建空 vector                           │
//  │  vec![1, 2, 3]              用宏创建带初始值的 vector                │
//  │  vec![0; n]                 创建 n 个相同值的 vector                 │
//  │  Vec::with_capacity(n)      预分配容量                               │
//  │  iter.collect()             从迭代器收集                             │
//  │  Vec::from(array)           从数组转换                               │
//  └──────────────────────────────────────────────────────────────────────┘
//
//  ┌─ 添加 ───────────────────────────────────────────────────────────────┐
//  │  push(val)                  尾部添加                O(1) 均摊       │
//  │  insert(idx, val)           指定位置插入            O(n)            │
//  │  append(&mut other)         合并另一个 vector       O(n)            │
//  │  extend_from_slice(&s)      从切片扩展              O(n)            │
//  │  extend(iter)               从迭代器扩展            O(n)            │
//  └──────────────────────────────────────────────────────────────────────┘
//
//  ┌─ 删除 ───────────────────────────────────────────────────────────────┐
//  │  pop()                      移除末尾 → Option<T>   O(1)            │
//  │  remove(idx)                移除指定索引 → T       O(n)            │
//  │  swap_remove(idx)           交换末尾并移除 → T     O(1) 不保序     │
//  │  clear()                    清空（保留容量）       O(n)            │
//  │  truncate(len)              截断到指定长度         O(n)            │
//  │  drain(range)               移除范围并返回迭代器   O(n)            │
//  │  retain(|x| bool)           只保留满足条件的       O(n)            │
//  │  splice(range, iter)        替换范围内的元素       O(n)            │
//  │  split_off(at)              从 at 处分割成两个     O(n)            │
//  └──────────────────────────────────────────────────────────────────────┘
//
//  ┌─ 访问 ───────────────────────────────────────────────────────────────┐
//  │  v[i] / &v[i]              索引访问（越界 panic）  O(1)            │
//  │  v.get(i)                  安全访问 → Option<&T>   O(1)            │
//  │  v.first() / v.last()     首/尾元素 → Option<&T>  O(1)            │
//  │  v.len()                   长度                    O(1)            │
//  │  v.is_empty()              是否为空                O(1)            │
//  │  v.contains(&val)          是否包含                O(n)            │
//  └──────────────────────────────────────────────────────────────────────┘
//
//  ┌─ 排序与查找 ─────────────────────────────────────────────────────────┐
//  │  sort()                     稳定排序               O(n log n)      │
//  │  sort_unstable()            不稳定排序（更快）     O(n log n)      │
//  │  sort_by(|a,b| cmp)        自定义排序              O(n log n)      │
//  │  sort_by_key(|x| key)      按键排序               O(n log n)      │
//  │  reverse()                  反转                   O(n)            │
//  │  binary_search(&val)        二分查找（需已排序）   O(log n)        │
//  │  iter().position(|x| bool) 找索引                  O(n)            │
//  │  iter().find(|x| bool)     找元素                  O(n)            │
//  └──────────────────────────────────────────────────────────────────────┘
//
//  ┌─ 去重 ───────────────────────────────────────────────────────────────┐
//  │  dedup()                    去除连续重复           O(n)            │
//  │  dedup_by(|a,b| bool)      自定义去重              O(n)            │
//  │  dedup_by_key(|x| key)     按键去重               O(n)            │
//  └──────────────────────────────────────────────────────────────────────┘
//
//  ┌─ 切片与分块 ─────────────────────────────────────────────────────────┐
//  │  &v[a..b]                  切片引用                O(1)            │
//  │  chunks(n)                 按 n 分块               O(1) 创建       │
//  │  windows(n)                大小为 n 的滑动窗口     O(1) 创建       │
//  │  split_at(mid)             在 mid 处分割           O(1)            │
//  │  as_slice()                转为切片                O(1)            │
//  └──────────────────────────────────────────────────────────────────────┘
//
//  ┌─ 容量管理 ───────────────────────────────────────────────────────────┐
//  │  capacity()                 已分配容量              O(1)            │
//  │  reserve(n)                 预留至少 n 个空位       O(n) 最坏       │
//  │  reserve_exact(n)           精确预留                O(n) 最坏       │
//  │  shrink_to_fit()            收缩到 len             O(n) 最坏       │
//  │  shrink_to(n)               收缩到 max(n, len)    O(n) 最坏       │
//  └──────────────────────────────────────────────────────────────────────┘
//
//  ┌─ 迭代器方法 ─────────────────────────────────────────────────────────┐
//  │  iter()                     不可变迭代 (&T)                         │
//  │  iter_mut()                 可变迭代 (&mut T)                       │
//  │  into_iter()                消费迭代 (T)                            │
//  │  map(|x| ...)              变换                                     │
//  │  filter(|x| bool)          过滤                                     │
//  │  fold(init, |acc, x| ...)  折叠                                     │
//  │  sum() / product()         求和/求积                                │
//  │  max() / min()             最大/最小                                │
//  │  any(|x| bool)             存在满足条件的                           │
//  │  all(|x| bool)             全部满足条件                             │
//  │  zip(other)                配对                                     │
//  │  enumerate()               带索引                                   │
//  │  flatten()                  扁平化                                  │
//  │  partition(|x| bool)       分区                                     │
//  │  collect()                 收集                                     │
//  └──────────────────────────────────────────────────────────────────────┘
//
//  ┌─ 转换 ───────────────────────────────────────────────────────────────┐
//  │  as_slice() / as_mut_slice()    Vec<T> ↔ &[T]                      │
//  │  into_boxed_slice()             Vec<T> → Box<[T]>                  │
//  │  to_vec()                       &[T] → Vec<T>                      │
//  │  Vec::from(array)               [T; N] → Vec<T>                    │
//  │  v.try_into()                   Vec<T> → [T; N]                    │
//  │  String::from_utf8(v)           Vec<u8> → String                   │
//  │  s.into_bytes()                 String → Vec<u8>                   │
//  │  iter.collect()                 Iterator → Vec<T>                  │
//  └──────────────────────────────────────────────────────────────────────┘
