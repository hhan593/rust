use std::error::Error; // 引入错误 trait
use std::fs; // 引入文件系统库，用于读取文件

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容。? 是 Rust 的语法糖：如果成功则解包，如果失败则立即返回该错误
    let contents = fs::read_to_string(config.filename)?;

    // println!("文件内容如下：\n{contents}");
 for  line in  search(&config.query, &contents){
     println!("{}", line);
 }
    // 运行成功，返回 Ok 包含 unit 类型 ()
    Ok(())
}

/// 配置信息结构体
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    /// 构建 Config 实例的工厂函数
    /// 使用 Result 处理可能的错误（参数不足的情况）
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // 检查参数数量是否足够
        if args.len() < 3 {
            return Err("参数不足（需要查询字符串和文件名）");
        }

        // 使用 clone() 将参数所有权从引用的切片中转移到结构体字段中
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
//// 编译器不知道返回的 &str 是跟 query 混的，还是跟 contents 混的,所有要加生命周期参数
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line); // 这里的 line 是 contents 里的完整一行
        }
    }

    results
}

// 测试模块

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_arg() {
        let query = "duct";
        // 注意：这里的空格必须和 assert_eq! 里的右侧完全一致
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."], // 这里补上空格
            search(query, contents)
        );
    }
}
