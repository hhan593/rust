# Rust 学习项目集合

这是一个个人 Rust 学习仓库，包含多个独立 Cargo crate、章节练习、workspace 示例和中文学习文档。

## 目录结构

| 路径 | 说明 |
| --- | --- |
| `docs/` | Rust 学习文档和知识手册 |
| `rust_project/` | 按章节组织的系统化练习项目 |
| `workspaces/` | Cargo workspace 示例，包含 `add_one`、`add_two`、`adder` |
| 根目录各子目录 | 独立 Rust 练习 crate，可单独运行或检查 |

## 顶层练习 crate

### 入门与基础语法

| 项目 | 说明 |
| --- | --- |
| `hello_world` | Hello World 入门 |
| `guess` | 猜数字游戏 |
| `day` | 基础练习 |
| `functions` | 函数与控制流练习 |
| `xunhuan` | 循环练习 |
| `games` | 游戏相关练习 |

### 所有权、类型与模式匹配

| 项目 | 说明 |
| --- | --- |
| `ownership` | 所有权练习 |
| `slice` | 切片练习 |
| `structs` | 结构体练习 |
| `enums` | 枚举与 Option 练习 |
| `matchs` | 模式匹配练习 |
| `strings` | String 与字符串练习 |
| `lifetimes` | 生命周期练习 |
| `trait_generics_lifetime` | 泛型、Trait、生命周期综合练习 |

### 集合、迭代器与闭包

| 项目 | 说明 |
| --- | --- |
| `Vector` | Vec 练习 |
| `hash` | HashMap 与哈希集合练习 |
| `iterator` | 迭代器练习 |
| `closures` | 闭包练习 |

### 模块、测试与 Rust Book 项目

| 项目 | 说明 |
| --- | --- |
| `adder` | 测试示例 crate |
| `aggregator` | Trait 示例库 |
| `my_crate` | 库 crate 练习 |
| `restaurant` | 模块系统与可见性练习 |
| `pages` | 模块文件拆分练习 |
| `minigrep` | Rust Book minigrep 项目 |
| `minigrep_plus` | minigrep 增强版 |

### 智能指针、内存与并发

| 项目 | 说明 |
| --- | --- |
| `smartPointer` | 智能指针练习 |
| `deref` | Deref 练习 |
| `memory_leak` | Rc、RefCell 与内存泄漏练习 |
| `Cell` | Cell 与 RefCell 内部可变性练习 |
| `Arc` | Arc 练习 |
| `share_state` | 共享状态并发练习 |
| `channel` | 通道与消息传递练习 |
| `Threads` | 线程练习 |

### 异步、高级特性与面向对象模式

| 项目 | 说明 |
| --- | --- |
| `Future` | Future 练习 |
| `Stream` | Stream 练习 |
| `Pin` | Pin / Unpin 练习 |
| `async_test` | 异步练习 |
| `async-test` | 另一个异步练习 |
| `yibutrait` | 异步 Trait / Future trait 风格练习 |
| `superpower` | 高级 trait 与 unsafe 练习 |
| `gui` | GUI / trait object 练习 |
| `object` | 面向对象与 trait object 练习 |
| `Stete-patten` | 状态模式练习 |

## 常用命令

在单个练习项目中运行：

```bash
cargo run --manifest-path hello_world/Cargo.toml
cargo check --manifest-path ownership/Cargo.toml
cargo test --manifest-path minigrep_plus/Cargo.toml
```

检查 workspace 示例：

```bash
cargo check --manifest-path workspaces/Cargo.toml --workspace
cargo test --manifest-path workspaces/Cargo.toml --workspace
```

检查章节练习中的某个 crate：

```bash
cargo check --manifest-path rust_project/ch3/ch2_string_str/Cargo.toml
```

## 后续结构整理建议

- 统一目录和 package 命名，优先改成小写 snake_case 或 kebab-case。
- 逐个确认 `main copy.rs` 是备份还是可运行示例；有价值的示例可移到 `src/bin/` 或 `examples/`。
- 明确 `async_test` 和 `async-test` 的区别，必要时合并或改成更具体的名称。
- 评估是否新增根 workspace；建议先纳入稳定的顶层 crate，再逐步扩展到章节项目。
