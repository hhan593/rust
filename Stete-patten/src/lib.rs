// ==========================================
// 1. 上下文对象 (Context): Post 结构体定义
// ==========================================

// 定义一个公开的结构体 Post，代表博客文章
pub struct Post {
    // 字段 state:
    // - Option: 表示状态可能存在也可能暂时为空（在状态转换的瞬间）。
    // - Box: 智能指针，将数据存储在堆（Heap）上。因为不同状态（Draft, Published等）大小可能不同，
    //        且我们需要在运行时动态改变类型，所以必须用 Box 包裹。
    // - dyn State: 特质对象（Trait Object）。表示“任何实现了 State 特质的类型”。
    //              这允许我们在运行时多态地调用不同状态的方法。
    state: Option<Box<dyn State>>,

    // 字段 content:
    // 一个可变的字符串，用于存储文章的实际文本内容。
    // 它存储在栈上（String 内部管理堆内存），属于 Post 实例的一部分。
    content: String,
}

// ==========================================
// 2. Post 的方法实现 (impl Post)
// ==========================================

// 为 Post 结构体实现方法
impl Post {
    // 定义一个公开关联函数 new，作为构造函数。
    // 返回类型是 Post 的新实例。
    pub fn new() -> Post {
        // 创建并返回一个新的 Post 结构体实例
        Post {
            // 初始化 state 字段：
            // 1. Draft {}: 创建一个具体的 Draft 状态实例。
            // 2. Box::new(...): 将 Draft 实例移动到堆上，并返回一个指向它的智能指针。
            // 3. Some(...): 将这个指针包装进 Option，表示当前状态存在。
            // 此时，Post 的初始状态被设定为“草稿”。
            state: Some(Box::new(Draft {})),

            // 初始化 content 字段：
            // 创建一个空的 String 对象。
            content: String::new(),
        }
    }

    // 定义公开方法 add_text，用于向文章添加文本。
    // &mut self: 需要可变借用 Post 实例，因为我们要修改 content 字段。
    // text: &str: 接收一个字符串切片作为参数。
    pub fn add_text(&mut self, text: &str) {
        // push_str: 将传入的 text 追加到 self.content 字符串的末尾。
        // 这会修改 content 指向的堆内存中的数据。
        self.content.push_str(text);
    }

    // 定义公开方法 request_review，用于请求审核。
    // &mut self: 需要可变借用，因为我们要修改 state 字段（切换状态）。
    pub fn request_review(&mut self) {
        // if let Some(s) = self.state.take():
        // 这是状态转换的核心魔法：
        // 1. self.state.take():
        //    - 从 Option 中“取出”值（所有权转移给变量 s）。
        //    - 原位置 self.state 变为 None。
        //    - 这样做是为了获得旧状态对象 `s` 的完全所有权，以便消耗它。
        // 2. Some(s): 如果取出了值（正常情况下总是能取出），进入代码块。
        if let Some(s) = self.state.take() {
            // s.request_review():
            // - 调用旧状态对象 `s` 的 request_review 方法。
            // - 注意：这里发生了“动态分发”。Rust 在运行时检查 `s` 具体是 Draft 还是其他，
            //   然后跳转到对应的实现代码。
            // - 该方法会消耗 `s`（旧状态销毁），并返回一个新的 Box<dyn State>（新状态）。
            // self.state = Some(...):
            // 将返回的新状态对象重新放入 state 字段，完成状态切换。
            self.state = Some(s.request_review());
        }
    }

    // 定义公开方法 approve，用于批准审核。
    // 逻辑与 request_review 几乎完全相同，只是调用的方法名不同。
    pub fn approve(&mut self) {
        // 同样，先拿走旧状态的所有权
        if let Some(s) = self.state.take() {
            // 调用旧状态的 approve 方法，获取新状态，并存回 state 字段
            self.state = Some(s.approve());
        }
    }

    // 定义公开方法 content，用于获取文章内容。
    // &self: 只需要不可变借用，因为我们只读取数据，不修改。
    // 返回类型 &str: 返回一个字符串切片（引用），避免复制整个字符串。
    pub fn content(&self) -> &str {
        // 逻辑判断：只有当状态是“已发布”时，才返回真实内容。

        // self.state.as_ref():
        // - 将 Option<Box<...>> 转换为 Option<&Box<...>>。
        // - 我们只是借用状态对象，而不是拿走它（因为方法是 &self）。
        // .unwrap():
        // - 解包 Option。假设逻辑正确，state 永远不为 None，否则程序会 panic。
        // .is_published():
        // - 调用状态对象的 is_published 方法。
        // - 再次发生动态分发：如果是 Published 结构体，返回 true；否则返回 false。
        if self.state.as_ref().unwrap().is_published() {
            // 如果已发布，返回 content 字段的引用
            &self.content
        } else {
            // 如果未发布（草稿或待审核），返回空字符串切片
            ""
        }
    }
}

// ==========================================
// 3. 状态接口定义 (Trait State)
// ==========================================

// 定义一个特质（接口）State。
// 注意：默认没有 pub，意味着它只在当前模块可见（对于内部状态模式通常足够）。
trait State {
    // 定义方法 request_review。
    // self: Box<Self>:
    // - 这是一个特殊的接收者语法。它表示该方法需要获取调用者（当前状态对象）的【所有权】。
    // - 调用后，当前的状态实例将被移动（Move），无法再被使用。
    // -> Box<dyn State>:
    // - 返回一个新的状态对象（装箱的特质对象）。
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // 定义方法 approve。
    // 同样需要获取所有权并返回新状态。
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // 定义辅助方法 is_published。
    // &self: 只需要借用，不需要销毁状态。
    // -> bool: 返回布尔值。
    // { false }: 提供默认实现，返回 false。
    // 这意味着只有 Published 状态需要重写这个方法，其他状态默认都是未发布。
    fn is_published(&self) -> bool {
        false
    }
}

// ==========================================
// 4. 具体状态实现：Draft (草稿)
// ==========================================

// 定义一个结构体 Draft，代表“草稿”状态。
// 它没有任何字段，只是一个标记类型。
struct Draft {}

// 为 Draft 实现 State 特质
impl State for Draft {
    // 实现 request_review 方法
    // self: Box<Self>: 接收当前的 Draft 实例所有权。
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // 打印日志，方便观察状态流转
        println!("状态变更: Draft -> PendingReview");

        // 核心逻辑：
        // 1. 当前的 `self` (Draft) 在这里生命周期结束，会被销毁。
        // 2. 创建一个新的 PendingReview 实例。
        // 3. 将其装箱并返回。
        // 这完成了从 Draft 到 PendingReview 的转换。
        Box::new(PendingReview {})
    }

    // 实现 approve 方法
    fn approve(self: Box<Self>) -> Box<dyn State> {
        // 打印日志：草稿不能直接批准
        println!("操作无效: 草稿不能直接批准");

        // 核心逻辑：
        // 保持状态不变。
        // 我们将收到的 `self` (Draft) 直接返回（重新装箱）。
        // 虽然经历了一次移动，但类型没变，逻辑上状态保持为 Draft。
        self
    }

    // 实现 is_published 方法
    // 覆盖默认实现（其实默认就是 false，这里显式写出更清晰）
    fn is_published(&self) -> bool {
        false
    }
}

// ==========================================
// 5. 具体状态实现：PendingReview (待审核)
// ==========================================

// 定义结构体 PendingReview，代表“待审核”状态
struct PendingReview {}

// 为 PendingReview 实现 State 特质
impl State for PendingReview {
    // 实现 request_review 方法
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // 打印日志：已经是待审核，重复操作无影响
        println!("状态保持: PendingReview");

        // 返回自己，状态保持为 PendingReview
        self
    }

    // 实现 approve 方法
    fn approve(self: Box<Self>) -> Box<dyn State> {
        // 打印日志：审核通过
        println!("状态变更: PendingReview -> Published");

        // 核心逻辑：
        // 1. 销毁当前的 PendingReview (`self`)。
        // 2. 创建并返回一个新的 Published 实例。
        Box::new(Published {})
    }

    // 实现 is_published 方法
    fn is_published(&self) -> bool {
        false
    }
}

// ==========================================
// 6. 具体状态实现：Published (已发布)
// ==========================================

// 定义结构体 Published，代表“已发布”状态
struct Published {}

// 为 Published 实现 State 特质
impl State for Published {
    // 实现 request_review 方法
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // 打印日志：已发布状态下请求审核，保持不变
        println!("状态保持: Published");
        self
    }

    // 实现 approve 方法
    fn approve(self: Box<Self>) -> Box<dyn State> {
        // 打印日志：已发布状态下再次批准，保持不变
        println!("状态保持: Published");
        self
    }

    // 实现 is_published 方法
    // 关键点：这里返回 true。
    // 这将使得 Post::content() 方法能够返回真实的文章內容。
    fn is_published(&self) -> bool {
        true
    }
}

// ==========================================
// 7. 主函数 (Entry Point)
// ==========================================

// 程序入口点
fn main() {
    // 调用 Post::new() 创建一个新帖子。
    // 此时内部状态是 Box<Draft>。
    let mut post = Post::new();

    // --- 阶段 1: 草稿 ---
    // 调用 add_text，向 content 字段追加字符串。
    post.add_text("I ate a salad for lunch today");

    // 调用 content() 方法。
    // 内部检查：state 是 Draft -> is_published() 返回 false -> 返回 ""。
    // assert_eq! 宏验证返回值是否等于空字符串，如果不等则程序崩溃。
    assert_eq!("", post.content());

    // --- 阶段 2: 请求审核 ---
    // 调用 request_review()。
    // 内部流程：take() 拿走 Draft -> 调用 Draft::request_review() -> 返回 Box<PendingReview> -> 存入 state。
    post.request_review();

    // 再次检查内容。
    // 内部检查：state 是 PendingReview -> is_published() 返回 false -> 返回 ""。
    assert_eq!("", post.content());

    // --- 阶段 3: 批准 ---
    // 调用 approve()。
    // 内部流程：take() 拿走 PendingReview -> 调用 PendingReview::approve() -> 返回 Box<Published> -> 存入 state。
    post.approve();

    // 再次检查内容。
    // 内部检查：state 是 Published -> is_published() 返回 true -> 返回 &self.content ("I ate a salad...")。
    // 验证成功，说明状态模式正常工作。
    assert_eq!("I ate a salad for lunch today", post.content());

    // --- 阶段 4: 边界测试 ---
    // 再次调用 approve()。
    // 内部流程：take() 拿走 Published -> 调用 Published::approve() -> 返回 Box<Published> (自己)。
    // 状态应该保持为 Published，不会崩塌。
    post.approve();

    // 验证内容依然可见。
    assert_eq!("I ate a salad for lunch today", post.content());

    // 如果代码运行到这里没有 panic，说明所有断言通过。
    // (在实际运行中，cargo run 不会输出任何错误信息，只会输出 println 的日志)
}