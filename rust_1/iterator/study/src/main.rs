//Iterator Trait 定义
pub trait Iterator {
    type Item; //关联类型：迭代器产生的元素类型、

    fn next(&mut self) -> Option<Self::Item>; //返回Option<Self::Item>类型

    // `Vec<i32>::iter()` 的 `Item` 是 `&i32`
    //  `Vec<i32>::into_iter()` 的 `Item` 是 `i32`

    // 返回 Some(item) 表示还有元素
    // 返回 None 表示迭代结束

    // 标准库提供了大量默认方法（map、filter、collect 等），这些方法依赖于 next 方法
}

fn main() {
    let arr = vec![1, 2, 3];
    let arr1 = vec![1, 2, 3];

    for val in arr {
        // println!("{}", val);
    }
    // 对每个元素执行副作用（如打印），应该用 for_each，它是立即执行的
    //map， 必须在后面接一个消费操作，比如 collect、count、for 循环等
    // arr1.iter().for_each(|x| println!("{}", x));

    // 函数式编程风格：函数式编程风格强调使用纯函数（Pure Function）和不可变数据（Immutable Data）
    let nums = vec![1, 2, 3, 4, 5];
    let mut sum = 0;
    for val in &nums {
        if (*val % 2 == 0) {
            sum += val;
        }
    }
    // println!("{}", sum);

    // 函数式编程风格：函数式编程风格强调使用纯函数（Pure Function）和不可变数据（Immutable Data），更清晰更可组合
    let sum: i32 = nums.iter().filter(|&num| num % 2 == 0).sum();

    // println!("函数风格{}", sum)

    // 2. 创建迭代器的三种方式
    // iter(); 不可变借用

    let nums = vec![1, 2, 3, 4, 5];
    for &val in nums.iter() {
        println!("{}", val);
    }
    println!("iter() 不可变借用 ，{:?}", nums); //此时，nums 仍然可用

    // iter_mut(); 可变借用

    let mut nums = vec![1, 2, 3, 4, 5];
    for val in nums.iter_mut() {
        *val += 1; //可以修改每个元素
    }
    println!("iter_mut() 可变借用，{:?}", nums); //此时，nums 仍然可用

    // into_iter(); 消费（获取所有权）

    let nums = vec![1, 2, 3, 4, 5];
    for val in nums.into_iter() {
        println!("{}", val);
    }
    // println!("into_iter() 消费（获取所有权）, {:?}", nums); //此时，nums 不可用，此时的所有权已经转移

    // 三者场景对比

    // | 方法           | Item 类型  | 原集合可用？ | 适用场景  |
    // | ------------- | -------- | ------ | ----- |
    // | `iter()`      | `&T`     | ✅ 是    | 只读遍历  |
    // | `iter_mut()`  | `&mut T` | ✅ 是    | 修改元素  |
    // | `into_iter()` | `T`      | ❌ 否    | 获取所有权 |

    // ## 3. 消费适配器（Consuming Adaptors）：消费适配器会消耗掉迭代器，返回一个计算后的值，调用next()直至结束，返回最终结果
}
