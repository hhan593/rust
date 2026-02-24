use std::io::stdin;

fn main()  {
    // println!("Hello, world!");

    let mut msg = String::new();
    println!("请输入文字");

    stdin().read_line(&mut msg).unwrap();
    println!("文字是 {}",msg)

}

//crate

// library crate  // 一个
// binary crate  //可执行