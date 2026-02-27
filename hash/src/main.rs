// HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。HashMap 通过使用哈希函数来计算键的哈希值，并将其映射到一个桶中来存储键值对。
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50); // 正确：插入的是 i32 类型的值 50,键是 String 类型的 "Yellow"

    // 访问哈希 map 中的值，通过get 方法

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {}", score);
    // get 方法返回 Option<&V>，如果某个键在哈希 map 中没有对应的值，get 会返回 None。
    // 程序中通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>，
    // 接着调用 unwrap_or 在 scores 中没有该键所对应的项时将其设置为零

    // 遍历哈希集合

    for (key, value) in &scores {
        println!("{}: {}", key, value);
        //打印结果
        // Yellow: 50
        // Blue: 10
    }
    // 哈希 map 和所有权

    let filed_name = String::from("/tmp/test.txt");
    // let filed_value =String::from("/tmp/test.txt");

    let filed_value = 12;

    let mut filed_map = HashMap::new();
    filed_map.insert(filed_name, filed_value);
    println!("map: {:?}", filed_value);
    // filed_name 和 filed_value 被移动到 filed_map 中，此时key和value都为String类型，不可以使用
    //但是当遇到实现了Copy trait的类型时，值会被复制而不是移动，所以在这种情况下，filed_value仍然可以使用，因为i32类型实现了Copy trait。

    //更新哈希 map
    // 可以选择完全无视旧值并用新值代替旧值。可以选择保留旧值而忽略新值，并只在键没有对应值时增加新值。或者可以结合新旧两值。
    //唯一的键只能对一个值，但是值可能同时对应好几个键，键1和键2都对应相同的值

    //覆盖一个值
    let mut map1 = HashMap::new();
    map1.insert(String::from("Blue"), 10);
    map1.insert(String::from("Blue"), 50);

    println!("{:?}", map1); //{"Blue": 50}原来的值 10 被覆盖了

    //只在键尚不存在时插入键值对 ,如果哈希 map 中键已经存在则不做任何操作；如果不存在则连同值一块插入。

    map1.insert(String::from("Red"), 10);
    map1.entry(String::from("Yellow")).or_insert(10);
    println!("{:?}", map1);

    //根据旧值更新一个值

    // 统计文本中各个单词出现的次数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // 使用 entry() 方法获取键的入口，若键不存在则插入默认值 0，然后获取可变引用
        // 将计数值加 1，实现对单词频率的统计
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    //哈希函数
}

// 总结
// vector、字符串和哈希 map 会在你的程序需要储存、访问和修改数据时帮助你。这里有一些你应该能够解决的练习问题：

// 给定一组整数，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（出现次数最多的值；在这里哈希 map 会很有帮助）。
// 将字符串转换为 pig latin。也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 ay，所以 first 会变成 irst-fay。元音字母开头的单词则在结尾增加 hay（apple 会变成 apple-hay）。请注意 UTF-8 编码的细节！
// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
