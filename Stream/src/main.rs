use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        /* --- 基础篇：从迭代器创建流 --- */
        // let values = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        // // map 是惰性的，直到流被 poll 才会执行
        // let iter = values.iter().map(|n| n * 2);
        // // stream_from_iter 将同步迭代器转换为异步流
        // let mut stream  = trpl::stream_from_iter(iter);
        //
        // // next() 返回一个 Future，所以需要 .await
        // // 当流结束时返回 None，退出循环
        // while let Some(value) = stream.next().await {
        //     println!("基础值: {}", value);
        // }

        /* --- 进阶篇：过滤流 --- */
        // let values = 1..101;
        // let iter = values.map(|num| num * 2);
        // let stream = trpl::stream_from_iter(iter);
        //
        // // filter 也是组合子，只有符合条件的 Item 会被传递给下游
        // let mut filtered = stream.filter(|val| val % 3 == 0 || val % 5 == 0);
        //
        // while let Some(value) = filtered.next().await {
        //     println!("过滤值: {}", value);
        // }

        /* --- 组合篇：超时处理 (Timeout) --- */
        // // 获取流后链式调用 .timeout()
        // // 注意：timeout 返回的是 Result<Item, Elapsed>
        // let mut message = pin!(get_message().timeout(Duration::from_millis(200)));
        //
        // while let Some(result) = message.next().await {
        //     match result {
        //         Ok(msg) => println!("Message: {}", msg),
        //         Err(_) => println!("Error: 超时了！"),
        //     }
        // }

        /* --- 实战篇：合并流 (Merge) --- */

        // 1. 处理消息流：加上 500ms 超时，并将 Result 转换为 String 统一类型
        // unwrap_or_else 确保了 messages 的 Item 类型是 String
        let messages = get_message()
            .timeout(Duration::from_millis(500))
            .map(|res| res.unwrap_or_else(|_| "⚠️ 消息读取超时!".to_string()));

        // 2. 处理间隔流：使用 map 将 u32 转换为 String
        // 核心点：merge 要求两个流的 Item 类型必须完全一致
        let intervals = get_intervals().map(|num| format!("🕒 计时器: {}", num));

        // 3. 合并流：merge 会同时监听两个流，谁先到就先处理谁
        //
        let merged = messages.merge(intervals);

        // 4. 固定内存 (Pinning)：
        // 像 merge 这种复杂的流包装器，在 await 过程中不能在内存中移动
        let mut stream = pin!(merged);

        // 5. 循环消费：只要两个流中有一个还没结束，这里就会持续运行
        while let Some(value) = stream.next().await {
            println!("收到数据: {}", value);
        }

        println!("所有流已关闭。");
    })
}

/// 模拟一个异步消息发送者
fn get_message() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    // 在后台任务中运行，模拟网络波动
    trpl::spawn_task(async move {
        let messages = vec!["Alpha", "Beta", "Gamma", "Delta"];
        for msg in messages {
            // 模拟不稳定的延迟
            trpl::sleep(Duration::from_millis(400)).await;
            // 如果发送失败（接收端已关），直接退出
            if tx.send(format!("数据包: {}", msg)).is_err() { break; }
        }
    });

    ReceiverStream::new(rx)
}

/// 模拟一个定时计数器
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut num = 0;
        loop {
            trpl::sleep(Duration::from_millis(300)).await;
            num += 1;
            // 发送心跳包
            if tx.send(num).is_err() { break; }
            if num >= 10 { break; } // 模拟运行10次后停止
        }
    });

    ReceiverStream::new(rx)
}