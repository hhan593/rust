pub mod test {
   pub fn conflow() {
        flow();
    }
    fn flow() {
        let num = 5;
        if num < 10 {
            println!("num is less than 10");
        } else {
            println!("num is greater than or equal to 10");
        }
    }
}
