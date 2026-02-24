# novel-tts

novel-tts 是一个专门为小说阅读设计的文本转语音（TTS）库。它基于 [kokoro-tts](https://github.com/mzdk100/kokoro) 引擎，提供了针对长文本（如小说章节）优化的流式处理功能。

## 功能特性

- 📚 专为小说阅读优化的TTS解决方案
- 🌊 流式音频处理，支持边生成边播放
- 🎵 支持多种语音选择
- 🔁 实时字符位置追踪，便于同步文本高亮
- ⏹️ 支持播放控制（暂停、取消）
- 📦 自动下载和管理TTS模型文件
- 🧵 异步API设计，适用于现代Rust应用

## 安装

在你的 [Cargo.toml](file:///Users/yexiyue/rust-project/TRNovel/crates/novel-tts/Cargo.toml) 中添加：

```toml
[dependencies]
novel-tts = { path = "crates/novel-tts" }
```

注意：请根据实际发布情况调整依赖配置。

## 快速开始

### 基本用法

```rust
use novel_tts::{NovelTTS, CheckpointModel, VoicesData, ChapterTTS};
use kokoro_tts::Voice;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化模型和语音数据
    let mut model = CheckpointModel::default();
    let mut voices = VoicesData::default();
    
    // 检查并下载必要的模型文件
    if !model.is_downloaded() {
        model.async_download(|downloaded, total| {
            println!("模型下载进度: {}/{}", downloaded, total);
        }).await?;
    }
    
    if !voices.is_downloaded() {
        voices.async_download(|downloaded, total| {
            println!("语音数据下载进度: {}/{}", downloaded, total);
        }).await?;
    }
    
    // 创建TTS实例
    let novel_tts = NovelTTS::new(&model, &voices).await?;
    let mut chapter_tts = novel_tts.chapter_tts();
    
    // 准备要转换的文本
    let text = "这是小说的第一段落。\n这是第二段落。".to_string();
    
    // 流式处理文本到音频
    let (audio_queue, mut position_rx) = chapter_tts.stream(
        text, 
        Voice::Zf006(1), 
        |error| eprintln!("TTS处理错误: {:?}", error)
    )?;
    
    // 监听字符位置更新
    tokio::spawn(async move {
        while let Some((start, end)) = position_rx.recv().await {
            println!("正在朗读字符位置: {} 到 {}", start, end);
        }
    });
    
    // 播放音频（需要rodio或其他音频播放库配合使用）
    // let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    // let sink = rodio::Sink::connect_new(stream_handle.mixer());
    // sink.append(audio_queue);
    // sink.sleep_until_end();
    
    Ok(())
}
```

## 待支持的功能

- [ ] 多人有声书：支持多个角色使用不同声音朗读
- [ ] 更丰富的语音控制选项
- [ ] 更灵活的文本预处理功能

## 许可证

MIT

## 贡献

欢迎提交Issue和Pull Request来帮助改善这个库。
