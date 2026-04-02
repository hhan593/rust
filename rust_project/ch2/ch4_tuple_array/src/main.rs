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

    //所有权

    let arr_item = [1, 2, 3, 4, 5]; //普通的栈上数组（[T; N]）通常实现了 Copy trait。因此，arr_item被复制到arr_item2，而不是移动。这意味着arr_item和arr_item2是两个独立的变量，可以同时使用。
    let arr_item2 = arr_item; //因为实现了Copy trait，所以arr_item2和arr_item是两个变量，两个变量可以同时使用
    println!("arr_item22222222222222222: {:?}", arr_item2);
    println!("arr_item33333333333333333: {:?}", arr_item);

    let arr_item3 = vec![1, 2, 3, 4, 5];
    let arr_item4 = arr_item3; //因为Vec<T>没有实现Copy trait，所以arr_item3被移动到arr_item4，arr_item3不再有效，不能再使用，否则会报错 borrow of moved value: `arr_item3` value borrowed here after move
                               // println!("arr_item3: {:?}", arr_item3);
    println!("arr_item4: {:?}", arr_item4);

    //move语义：当一个值被赋值给另一个变量时，原来的变量不再有效，新的变量成为该值的所有者。这种行为称为“移动”（move）。
    //当一个值被移动时，原来的变量不再有效，不能再使用，否则会报错 borrow of moved value: `arr_item3` value borrowed here after move

    let str = String::from("hello");
    let str2 = str; //str扥所有权被移动到str2，str不再有效
                    // println!("str: {}", str); //error borrow of moved value: `str`
    println!("str2: {}", str2);
}
