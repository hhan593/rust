pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
// trait 实现规则

// 合法示例：
//在本地类型Tweet实现标准款的Display trait
// 在标准库的Vec<T>上实现本地的Summary trait

//非法示例
// 在标准库的Vec<T>上实现标准款的Display trait，两者都来自标准库，没有来在本地的类型或者trait

//一致性和孤儿原则
