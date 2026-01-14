# Rust - 100天从新手到大师

> 作者：Jules (Agent)
> 参考：[Python - 100天从新手到大师](https://github.com/jackfrued/Python-100-Days)

Rust 是一门赋予每个人构建可靠且高效软件能力的语言。它以内存安全、高性能和并发安全性著称。本项目旨在通过100天的学习计划，帮助你从零开始掌握 Rust，并能应用它进行系统编程、Web开发等。

## 目录

### 第一阶段：Rust 语言基础 (Day 01 - 15)
在这个阶段，我们将学习 Rust 的基本语法和核心概念，特别是所有权系统，这是 Rust 最独特的功能。

- **Day 01**: 初识 Rust - 环境搭建、Hello World、Cargo 包管理工具
- **Day 02**: 变量与数据类型 - 不可变性、标量类型、复合类型
- **Day 03**: 函数与控制流 - 参数、返回值、if/else、循环
- **Day 04**: 所有权 (Ownership) - Rust 的核心内存管理机制
- **Day 05**: 引用与借用 (References & Borrowing) - 借用检查器
- **Day 06**:切片 (Slices) - 处理序列数据的引用
- **Day 07**: 结构体 (Structs) - 自定义数据类型
- **Day 08**: 枚举 (Enums) 与 模式匹配 (Match)
- **Day 09**: 控制流运算符 (if let)
- **Day 10**: 模块系统 (Modules) - 包、Crate、模块、路径
- **Day 11**: 常用集合 - Vector, String, HashMap
- **Day 12**: 错误处理 - Result, Option, panic!
- **Day 13**: 泛型 (Generics)
- **Day 14**: Trait (特征) - 定义共享行为
- **Day 15**: 生命周期 (Lifetimes) - 引用的有效性验证

### 第二阶段：进阶概念与标准库 (Day 16 - 30)
掌握 Rust 的中级概念，学习如何编写测试、处理 I/O 以及使用闭包和迭代器。

- **Day 16**:自动化测试 - 单元测试与集成测试
- **Day 17**: 命令行程序实例 - 构建 grep-lite
- **Day 18**: 闭包 (Closures)
- **Day 19**: 迭代器 (Iterators)
- **Day 20**: Cargo 进阶与 Crates.io 发布
- **Day 21**: 智能指针 (Smart Pointers) - Box, Rc, RefCell
- **Day 22**: 内部可变性 (Interior Mutability)
- **Day 23**: 引用循环与内存泄漏
- **Day 24**: 并发编程基础 - 线程 (Threads)
- **Day 25**: 消息传递 (Message Passing) - Channels
- **Day 26**: 共享状态并发 - Mutex, Arc
- **Day 27**: 面向对象特性 - Trait 对象
- **Day 28**: 模式 (Patterns) 与 模式匹配详解
- **Day 29**: 高级特征 - 关联类型、默认参数
- **Day 30**: 高级类型 - Newtype 模式、类型别名

### 第三阶段：高级特性与底层编程 (Day 31 - 45)
深入 Rust 的底层，学习不安全 Rust、宏以及 FFI。

- **Day 31**: Unsafe Rust - 解引用裸指针、调用不安全函数
- **Day 32**: 高级 Trait -父 Trait、Newtype
- **Day 33**: 宏 (Macros) - 声明式宏
- **Day 34**: 过程宏 (Procedural Macros)
- **Day 35**: FFI - 与 C 语言交互
- **Day 36**: Rust 异步编程基础 - async/await
- **Day 37**: Future 模型详解
- **Day 38**: Tokio 运行时入门
- **Day 39**: 异步 I/O 操作
- **Day 40**:构建异步 Web Server (底层实现)
- **Day 41-45**: 阶段项目 - 构建一个简易的多线程 Web 服务器

### 第四阶段：Web 开发与生态系统 (Day 46 - 75)
利用 Rust 强大的生态系统进行实际应用开发，重点在 Web 后端。

- **Day 46-50**: Web 框架 Axum 入门
- **Day 51-55**: 数据库交互 - SQLx 与 Postgres
- **Day 56-60**: 身份认证与授权 (JWT)
- **Day 61-65**: RESTful API 设计与实现
- **Day 66-70**: 使用 Serde 进行序列化与反序列化
- **Day 71-75**: WebAssembly (WASM) 入门 - Yew 或 Leptos 框架

### 第五阶段：系统编程与实战项目 (Day 76 - 100)
挑战复杂的系统级项目，巩固所学知识。

- **Day 76-80**: 命令行工具 (CLI) 开发 - 使用 Clap
- **Day 81-85**: 嵌入式 Rust 简介 (Optional) / 或高性能网络服务
- **Day 86-90**: 区块链基础概念实现 (简易版)
- **Day 91-100**: 最终项目 - 分布式键值存储系统 (Distributed Key-Value Store)

## 如何使用
建议按照天数顺序学习，每天阅读相关概念，并亲手编写代码。

## 环境要求
- Rust Stable
- VS Code (推荐插件: rust-analyzer, crates, dependi)
