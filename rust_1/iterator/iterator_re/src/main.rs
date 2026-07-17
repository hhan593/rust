// &[i32] &Vec

fn sum_with_loop(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in arr {
        sum += item
    }
    sum
}

fn sum_with_iter(arr: &[i32]) -> i32 {
    let sum = arr.iter().sum();
    sum
}
fn main() {
    const ARRAY_SIZE: usize = 1_000;
    let array: Vec<i32> = (1..=ARRAY_SIZE as i32).collect();
    let sum1 = sum_with_loop(&array);
    println!("{}", sum1);
    let sum2 = sum_with_iter(&array);
    println!("{}", sum2)
}
