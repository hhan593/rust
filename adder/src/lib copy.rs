// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]

    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    pub fn add_two(a: usize) -> usize {
        a + 3
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 5);
        // assert_eq宏运行测试时，会输出以下错误信息
        //assertion `left == right` failed
        //   left: 5
        //  right: 4
    }

    pub fn greeting(name: &str) -> String {
        format!("Hello {name}!")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn greeting_contains_name() {
            let result = greeting("Carol");
            assert!(result.contains("Carol"));
            //contains 是 String 类型（或者说 &str 字符串切片）自带的一个方法。它的作用非常直观：检查一个字符串是否包含另一个子字符串
        }
    }
}
// aeeert_eq宏和assert_ne宏
