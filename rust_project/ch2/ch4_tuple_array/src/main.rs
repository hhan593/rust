fn main() {
    //元组
    let tup = (0, "hello", 3.1);
    println!("{}", tup.0); //获取数据 tup.index即可拿到数据
    println!("{}", tup.1);
    println!("{}", tup.2);

    let mut tup2 = (0, "we");
    println!("tup.2 is {}", tup.2);
    tup2.0 = 89;
    println!("{}", tup2.0);
    // 空元组

    let tup3 = ();

    print!("{:?}", tup3);
}
