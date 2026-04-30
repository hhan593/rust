//返回值和返回引用
// 返回值和返回引用的区别在于，返回值会将数据复制一份，而返回引用则是返回数据的地址。返回引用可以避免不必要的数据复制，提高性能，但需要注意生命周期和所有权的问题。

fn func_return_value() -> i32 {
    return 10;
}

fn func_return_ref() -> &i32 {
    let x = 10;
    return &x;
}

fn func_non_copy_back() -> String {
    let s = String::from("hello");
    s
}
fn main() {
    let x = func_return_value();
    println!("x = {x}");
    let s = func_return_ref();
    println!("s = {s}");
}
