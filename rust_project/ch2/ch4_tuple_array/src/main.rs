fn main() {
    //元组：存储多个不同数据类型的数据
    let tup = (0, "hello", 3.1);
    println!("{}", tup.0); //获取数据 tup.index即可拿到数据
    println!("{}", tup.1);
    println!("{}", tup.2);

    let mut tup2 = (0, "we");
    println!("tup.2 is {}", tup.2);
    tup2.0 = 89; //修改数据
    tup2.1 = "world"; //修改数据但是保持数据类型不变，否则会报错 tup2.1 = 89; //error mismatched types expected &str found integer
    println!("{}", tup2.0);
    println!("{}", tup2.1);
    // 空元组

    let tup3 = ();

    print!("{:?}", tup3);

    // array 数组：存储多个相同数据类型的数据

    let mut arr = [1, 2, 3, 4, 5];
    println!("{}", arr[0]); //获取数据 arr[index]即可拿到数据
    arr[0] = 89;
    println!("Array length: {} {}", arr.len(), arr[0]);

    for item in arr {
        print!("{}", item);
    }
}
