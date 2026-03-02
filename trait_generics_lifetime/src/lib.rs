/// Summary trait 定义了可汇总类型的行为
/// 实现此 trait 的类型必须提供一个 summarize 方法来生成摘要字符串
pub trait Summary {
    /// 为实现类型创建一个摘要表示
    ///
    /// # Arguments
    ///
    /// * `self` - 对当前实例的引用
    ///
    /// # Returns
    ///
    /// 返回代表实例摘要的字符串
    fn summarize(&self) -> String;
}

/// 新闻文章结构体
/// 表示一篇新闻文章，包含标题、地点、作者和内容
pub struct NewsArticle {
    pub headline: String, // 文章标题
    pub location: String, // 发布地点
    pub author: String,   // 作者
    pub content: String,  // 文章内容
}

/// 为 NewsArticle 实现 Summary trait
impl Summary for NewsArticle {
    /// 生成新闻文章的摘要，格式为 "标题, by 作者 (地点)"
    ///
    /// # Arguments
    ///
    /// * `self` - 对当前 NewsArticle 实例的引用
    ///
    /// # Returns
    ///
    /// 返回格式化的新闻文章摘要字符串
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

/// 社交帖子结构体
/// 表示社交媒体上的帖子，包含用户名、内容以及是否是回复或转发的信息
pub struct SocialPost {
    pub username: String, // 用户名
    pub content: String,  // 帖子内容
    pub reply: bool,      // 是否为回复
    pub repost: bool,     // 是否为转发
}

/// 为 SocialPost 实现 Summary trait
impl Summary for SocialPost {
    /// 生成社交帖子的摘要，格式为 "用户名: 内容"
    ///
    /// # Arguments
    ///
    /// * `self` - 对当前 SocialPost 实例的引用
    ///
    /// # Returns
    ///
    /// 返回格式化的社交帖子摘要字符串
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
