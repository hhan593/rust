use std::time::Duration;
use trpl;

fn main() {
    trpl::run( //block-on
        async {
            let  (tx,mut rx)  = trpl::channel();//异步版本

            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
                String::from("(all)"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(2000)).await;
            }
            while let Some(val) = rx.recv().await {
                println!("val = {}", val);
            }
        }
    )
}
