use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // 检查参数长度，防止 args[1] 越界崩溃
    if args.len() < 3 {
        println!("请提供两个 URL 参数");
        return;
    }

    trpl::run(async {
        // 传入 String 而非 &str，避免生命周期烦恼
        let title_fut_1 = page_title(args[1].clone());
        let title_fut_2 = page_title(args[2].clone());

        // race 返回的是 Either<T, T>，这里的 T 是 (String, Option<String>)
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(val) => val,
            Either::Right(val) => val,
        };

        println!("Winner URL: {url}");

        match maybe_title {
            Some(title) => println!("Title: {title}"),
            None => println!("No title found at {url}"),
        }
    });
}

// 修改函数签名，返回元组 (URL, Title)
async fn page_title(url: String) -> (String, Option<String>) {
    let response = trpl::get(&url).await;
    let response_text = response.text().await;

    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    (url, title)
}