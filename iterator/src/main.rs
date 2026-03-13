//迭代器

fn main() {
    //：调用 map 方法创建一个新迭代器，接着调用 collect 方法消费新迭代器并创建一个 vector
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1); // 代码实际上并没有做任何事；所指定的闭包从未被调用过。警告提醒了我们原因所在：迭代器适配器是惰性的，因此我们需要在此处消费迭代器
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // 这里我们消费了迭代器

    assert_eq!(v2, vec![2, 3, 4]);
}
