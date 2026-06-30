fn func_copy_back() -> i32 {
    let num = 45;
    num
}

fn func_no_copy_backk() -> String {
    let s = String::from("hello");
    s
}
fn get_guess(mark: i32) -> &'static str {
    if (mark == 42) {
        "😃"
    } else {
        "😭"
    }
}
fn main() {
    let i = func_copy_back();
    println!("{}", i);
    let s = func_no_copy_backk();
    println!("{}", s);

    println!("{}", get_guess(i));
}