// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     route(four);
//     route(six);
// }

// fn route(ip_kind: IpAddrKind) {
//     println!("route: {:?}", ip_kind);
// }
/// 定义IP地址类型枚举，包含IPv4和IPv6两种类型
/// IPv4使用四个u8值表示地址
/// IPv6使用字符串表示地址
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    /// 获取IP地址的类型名称
    ///
    /// # 参数
    /// * `self` - IP地址实例的引用
    ///
    /// # 返回值
    /// 返回一个字符串切片，表示IP地址类型（"IPv4"或"IPv6"）
    fn get_ip_kind(&self) -> &str {
        match self {
            IpAddrKind::V4(..) => {
                println!("IPv4 address: {}.", self.get_ip_addr());
                "IPv4"
            }
            IpAddrKind::V6(..) => {
                println!("IPv6 address: {}", self.get_ip_addr());
                "IPv6"
            }
        }
    }

    /// 获取IP地址的字符串表示
    ///
    /// # 参数
    /// * `self` - IP地址实例的引用
    ///
    /// # 返回值
    /// 返回一个String，表示完整的IP地址
    fn get_ip_addr(&self) -> String {
        match self {
            IpAddrKind::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
            IpAddrKind::V6(s) => s.to_string(),
        }
    }
}
fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    let addr1 = home.get_ip_kind();
    let addr2 = loopback.get_ip_addr();
    println!("{}", addr1);
    println!("{}", addr2);
    let addr3 = loopback.get_ip_kind();
    let addr4 = loopback.get_ip_addr();
    println!("{}", addr3);
    println!("{}", addr4);
}
