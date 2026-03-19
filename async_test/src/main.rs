use trpl;
use std::time::Duration;



fn main() {
    trpl::block_on(async{
    // let fut_1 =    async  {
    //         for i in 1..10 {
    //             println!("hi number {} from the spawned task!", i);
    //             trpl::sleep(Duration::from_secs(1)).await;
    //         }
    //     };
    //   let fut_2 = async {
    //       for i in 1..5 {
    //           println!("hi number {i} from the second task!");
    //           trpl::sleep(Duration::from_millis(500)).await;
    //       }
    //   };
    //     trpl::join(fut_1, fut_2).await;

        // ***通过消息传递在两个任务之间发送数据
        //
        // let (tx,mut rx)  =trpl::channel(); //trpl::channel，用于线程的多生产者、单消费者信道 API 的异步版本
        //
        // let val = String::from("hi number");
        // tx.send(val).unwrap();
        // let received = rx.recv().await.unwrap();
        // println!("received {}", received);


        // 通过异步信道发送和接收多个消息并在每个消息之间通过 `await` 休眠
        let (tx, mut rx) = trpl::channel();
        //
        // let vals = vec![
        //     String::from("hi"),
        //     String::from("from"),
        //     String::from("the"),
        //     String::from("future"),
        // ];
        //
        // for val in vals {
        //     tx.send(val).unwrap();
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }
        //
        // while let Some(value) = rx.recv().await {
        //     println!("received '{value}'");
        // }
//

        //
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1000)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;

    });





}
