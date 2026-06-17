struct Person<'a> {
    name: &'a str, //如果为&str，则必须标注生命周期,<'a>表示这个name和Person的生命周期相同
    age: i32,
    color: String,
}

fn print_name(name: &str) {
    println!("name: {}", name);
}
fn main() {
    //String &str
    let mut s = String::from("hello C++");
    //to_string 和to_owned两种方法
    let course = "Rust".to_owned();
    let name = s.replace("hello", "hi");
    println!("{} {}", name, course);

    let rust = "/x52/75/733/74";
    println!("rust: {rust}");
    //String &str

    let name = "hello";
    let color = "red".to_string();

    let person = Person {
        name: name,
        age: 18,
        color: color,
    };
}
