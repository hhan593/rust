/// 主函数 - 演示Rust的所有权和引用概念
///
/// 此函数演示了两种不同的数据类型（i32和String）在解引用和克隆时的不同行为：
/// 1. 对于Copy类型的值（如i32），可以通过解引用来获取值的副本
/// 2. 对于非Copy类型的值（如String），需要使用clone()方法来创建新的实例
fn main() {
    // 演示对基本类型i32的引用和解引用
    let v: Vec<i32> = vec![1, 2, 3];
    let n_ref = &v[0];
    let n: i32 = *n_ref;

    // 演示对复杂类型String的引用和克隆
    let v: Vec<String> = vec![String::from("hello_world")];
    let s_ref = &v[0];
    let s: String = s_ref.clone();
    print!("{}", s);
    println!(" - {}", n);

    let mut name = (String::from("hello"), String::from("world"));
    let first_name_ref = &name.0;
    // let first_name_ref = get_first(&name);
    name.1.push_str("hh");
    println!("first_name_ref: {}, name: {:?}", first_name_ref, name.1);
}
// fn get_first(name: &(String, String)) -> &String {
//     &name.0
// }
