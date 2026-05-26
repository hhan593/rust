fn main() {
    println!("Hello, world!");
    let nice_sum: i64 = 2020; //这个类型是可以自己推导的，而且这时候是不可以变的
    let _a = 2020; //m没有用的变量他也会提示警告，可以加一个_来消除这个警告
    //举个例子
    // nice_sum += 1; //这个时候就会报错，因为nice_sum是不可变的 cannot assign twice to immutable variable
    println!("nice sum: {}", nice_sum);
    println!("{} nice-sum", nice_sum);

    let mut sum = 27; // c此时这个sum变量是可变的
    sum += nice_sum;
    println!("{} nice-sum", sum);

    let mut sad = 90;

    let _str = "hello";
    // 命名空间
    {
        let sad = 2020;
        println!("nice-sum: {}", sad);
    } //结束的时候这个大括号的sum就被销毁了，此时在外面是没办法访问到的
    // println!("sad: {}", sad);
    sad += sad;
    print!("{sad }");

    let mut str = 1;
    print!("str: {}", str); //原来是字符串但是现在是i32，这个是可以重新赋值的，叫变量遮蔽，但是如果没有加mut关键字就不可以改变，而且要是同类型的

    //改变
    //  str = "dhsdh";//此时就是不行的，不是同个类型
    str = 2; //此时还是可以的是因为str的类型没有变，都是i32的
    println!("str: {}", str);
}
