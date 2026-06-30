struct MyString<'q> {
    val: &'q str,
}
impl<'q> MyString<'q> {
    fn get_length(&self) -> usize {
        self.val.len()
    }
    fn modify_val(&mut self, val: &'q str) {
        self.val = val;
    }
}

fn main() {
    let str = String::from("value");
    let mut x = MyString { val: str.as_str() };

    println!("{}",
             x.get_length()
    );
    x.modify_val("value");
}


