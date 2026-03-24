use std::pin::Pin;
use std::task::{Context, Poll};

fn main() {
    println!("Hello, world!");
}

/// `Feture` trait 是一个异步执行的抽象接口
/// 类似于标准库中的 `Future` trait，用于表示可能尚未完成的异步计算
/// 
/// # 关联类型
/// 
/// * `Output` - 异步计算完成后返回的结果类型
/// 
/// # 方法
/// 
/// * `poll` - 尝试检查异步计算是否已完成，如果完成则返回结果
pub trait Feture {
    /// 异步计算完成后的输出类型
    type Output;
    
    /// 尝试轮询此异步任务的进度
    /// 
    /// # 参数
    /// 
    /// * `self` - trait 实现类型的 Pin 指针的可变引用
    /// * `cx` - 任务上下文，提供对 Waker 的访问，可以唤醒当前任务
    /// 
    /// # 返回值
    /// 
    /// * `Poll::Ready(output)` - 计算已完成，返回结果值
    /// * `Poll::Pending` - 计算尚未完成，等待下一次唤醒
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
//异步涉及的trait
//Future trait