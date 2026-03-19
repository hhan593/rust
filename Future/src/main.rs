// 引入 trpl 库中的类型
// Either: 用于表示两种可能结果之一的枚举（左或右），常用于处理竞争条件
// Html: 用于解析 HTML 文档的工具
use trpl::{Either, Html};

// #[tokio::main] // 注意：如果 trpl::run 内部封装了运行时，这里可能不需要，但通常建议显式指定
fn main() {
    // 收集命令行参数
    // args[0] 是程序名称，args[1] 和 args[2] 应该是用户提供的两个 URL
    let args: Vec<String> = std::env::args().collect();

    // 【安全检查】防止用户只提供了 0 个或 1 个 URL，导致后续访问 args[1] 或 args[2] 时 panic
    if args.len() < 3 {
        eprintln!("用法: {} <URL1> <URL2>", args.get(0).unwrap_or(&"program".to_string()));
        eprintln!("请提供两个 URL 参数进行竞速请求。");
        return;
    }

    // 启动异步运行时
    // trpl::run 是一个辅助函数，用于阻塞当前线程直到内部的 async block 完成
    trpl::run(async {
        // 准备两个异步任务 (Futures)
        // 我们克隆 String 以避免所有权转移导致的生命周期问题，让每个任务拥有自己的 URL 副本
        let title_fut_1 = page_title(args[1].clone());
        let title_fut_2 = page_title(args[2].clone());

        // 【核心逻辑】竞速 (Race)
        // trpl::race 会同时执行两个 future。
        // 一旦其中一个完成（无论成功与否，取决于实现，通常是第一个返回结果的），它就立即返回该结果，并取消另一个任务。
        // 返回值是 Either<A, B> 枚举：
        // - Either::Left(val): 表示第一个参数 (title_fut_1) 赢了
        // - Either::Right(val): 表示第二个参数 (title_fut_2) 赢了
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(val) => {
                println!("🏆 第一个 URL 获胜!");
                val
            },
            Either::Right(val) => {
                println!("🏆 第二个 URL 获胜!");
                val
            },
        };

        // 输出获胜者的 URL
        println!("Winner URL: {url}");

        // 处理标题结果
        match maybe_title {
            Some(title) => println!("📄 Title: {title}"),
            None => println!("⚠️  No title found at {url} (可能是页面没有 <title> 标签或解析失败)"),
        }
    });
}

/// 异步函数：获取网页标题
///
/// # 参数
/// * `url`: 目标网址 (String)
///
/// # 返回
/// * `(String, Option<String>)`: 返回元组，包含原始 URL 和找到的标题（如果有）
///
/// # 逻辑
/// 1. 发送 HTTP GET 请求
/// 2. 获取响应文本
/// 3. 解析 HTML 并提取 <title> 标签内容
async fn page_title(url: String) -> (String, Option<String>) {
    // 发送异步 GET 请求
    // 注意：如果请求失败，这里的 .await 可能会 panic 或者需要更完善的错误处理
    // 在实际生产中，通常会将返回类型改为 Result<(String, Option<String>), Error>
    let response = trpl::get(&url).await;

    // 获取响应体文本
    let response_text = response.text().await;

    // 解析 HTML
    // Html::parse 将字符串解析为可查询的文档对象
    // .select_first("title") 尝试查找第一个 <title> 标签
    // .map(...) 如果找到了标签，提取其内部文本；如果没找到，保持为 None
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    // 返回原始 URL 和解析出的标题
    (url, title)
}