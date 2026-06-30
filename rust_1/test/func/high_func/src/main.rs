fn main() {
    let result = apply(|x| x + 10, 5);
    println!("{}", result); // 15
    let result = apply_twice(|x| x * 5, 5);
    println!("{}", result);
    let result = apply_twice1(|x| x * 5, 5);
    println!("{}", result);


    //使用map
    let words = vec![
        String::from("rust"),
        String::from("closure"),
        String::from("iterator"),
    ];
    let lens = words.iter().map(|x| x.len()).collect::<Vec<_>>();
    println!("{:?}", lens);

    //筛选出长度大于 4 的字符串：
    let words = vec!["rust", "book", "closure", "iterator"];
    let vecs: Vec<&str> = words
        .into_iter()
        .filter(|x| x.len() > 4)
        .collect();
    println!("{:?}", vecs);

    //使用fold
    let nums = vec![2, 3, 4];

    let total = nums.iter().fold(1, |acc, x| acc * x);
    println!("{:?}", total);
}
// 实现一个函数，接收一个闭包和一个数字，把数字传给闭包。
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

//练习 2：实现 `apply_twice`

fn apply_twice<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(value))
}

fn apply_twice1<F>(mut f: F, value: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    let first = f(value);
    f(first)
}
// 练习 6：返回闭包
