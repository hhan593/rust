fn main() {
    collect();

    //sum()/ product()  聚合运算
    let nums = vec![1, 2, 3, 4, 5];

    let total: i32 = nums.iter().sum(); //sum() 返回迭代器的和
    let product: i32 = nums.iter().product(); //product() 返回迭代器的积
}

//collect 收集为集合
fn collect() {
    let nums = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect(); //collect 将迭代器收集为集合
    // [2, 4, 6, 8, 10]
    println!("{:?}", doubled);
    // 也可以收集为其他类型
    use std::collections::HashSet;
    let set: HashSet<i32> = nums.iter().copied().collect();
    println!("{:?}", set);
    // 从范围创建
    let v: Vec<i32> = (0..10).filter(|x| x % 2 == 0).collect();
    // [0, 2, 4, 6, 8]
    println!("{:?}", v);
}
