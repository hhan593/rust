static COUNTER: i32 = 0;
fn main() {
    const MAX_POINTS: i32 = 100_000;
    let mut x = 5;
    let x1 = "hello";
    let x = x1;

    {
        let x = 6;
        println!("The value of x is: {}", x);
    }
    unsafe {
        // COUNTER = 2;
        println!("{}", COUNTER)
    }
}
