fn main() {
    println!("Hello, world!");
    let user1 = User {
        email: String::from("<EMAIL>"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!(
        "{}-{}-{}-{}",
        user1.email,
        user1.username,
        user1.active,
        user1.sign_in_count //<EMAIL>-someusername123-true-1
    );
    // let mut user2 = User {
    //     email: String::from("<EMAIL>"),
    //     ..user1
    // };
    let user2 = build_user(String::from("<EMAIL>"), String::from("someusername123"));
    // println!(
    //     "{}@@{}@@{}@@{}@@",
    //     user2.email,
    //     user2.username,
    //     user2.active,
    //     user2.sign_in_count //<EMAIL>-someusername123-true-1
    // );

    println!("{}", user1.username);
    let black = Color(String::from("black"), 0, 0);
    let origin = Point(0, 0, String::from("origin"));

    println!("{:?}", black.0);
    println!("{:?}", origin.0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
struct Color(String, i32, i32);
struct Point(i32, i32, String);
