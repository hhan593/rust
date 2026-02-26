fn main() {
    let coin = Coin::Penny;
    let value = match_coin(coin);
    println!("The value of the coin is {} cents.", value);
}
enum Coin {
    Penny,   // 1美分
    Nickel,  // 5美分
    Dime,    // 10美分
    Quarter, // 25美分
}
// 这个函数的返回值类型为u8，表示硬币的面值（以美分为单位）所以每个分支都返回一个u8类型的值
//      match coin {
// 15 |         Coin::Penny => "1",
//    |                        ^^^ expected `u8`, found `&str`
fn match_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
