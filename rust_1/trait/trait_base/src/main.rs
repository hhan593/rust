// Trait 定义了一组方法签名，描述某种类型应该具有的行为。类似于其他语言中的接口（interface），但功能更强大。

trait Greeter {
    fn greer(&self) -> String {
        String::from("Hello, world!")
    }
    fn hello(&self) -> String {
        String::from("Hello, world!")
    }
}

struct Person {
    name: String,
    age: u32,
}

//为 Person 实现 Greeter 特征
impl Greeter for Person {
    fn greer(&self) -> String {
        format!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        )
    }
}

fn main() {
    let greeter = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{}", greeter.greer());
    println!("{}", greeter.hello());
}
