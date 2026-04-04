fn main() {
    //元组：存储多个不同数据类型的数据
    let tup = (0, "hello", 3.1);
    println!("{}", tup.0); //获取数据 tup.index即可拿到数据
    println!("{}", tup.1);
    println!("{}", tup.2);

    let mut tup2 = (0, "we");//可变的元组 使用mut来标识
    println!("tup.2 is {}", tup.2);
    tup2.0 = 89; //修改数据
    tup2.1 = "world"; //修改数据但是保持数据类型不变，否则会报错 tup2.1 = 89; //error mismatched types expected &str found integer
    println!("{}", tup2.0);
    println!("{}", tup2.1); //元组获取元素 tup.index
                            // 空元组

    let tup3 = (); //完全不占据任何空间，经常用作函数没有返回值的时候使用

    print!(" tup3 {:?}", tup3);

    // array 数组：存储多个相同数据类型的数据

    let mut arr = [1, 2, 3, 4, 5]; //所有的元素都列出来
    let arr2 = [5; 3]; //[value;size] 值和size
    println!("{}", arr[0]); //获取数据 arr[index]即可拿到数据
    arr[0] = 89;
    println!("Array length: {} {}", arr.len(), arr[0]);

    for item in arr {
        print!("{}", item);
    }
    for item in arr2 {
        println!("arr2的每一个 {}", item);
    }
    println!("{}", arr2.len());
}
//元组和数组的长度是固定的，但是数组可以动态扩容，元组不行，而且可以设置为可变的
