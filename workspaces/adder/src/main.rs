use add_one;
use add_two;

fn main() {
    let num: i32 = 2;
    println!("num is {}", num);
    println!("hello,world {num} plus one is {}", add_one::add_one(num));
    println!("hello,world {num} plus two is {}", add_two::add_two(num));
}
