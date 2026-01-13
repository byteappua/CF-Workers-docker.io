# Day 01 - 初识 Rust

## 1. Rust 简介
Rust 是一门赋予每个人构建可靠且高效软件能力的语言。
- **高性能**：Rust 速度惊人且内存利用率极高。由于没有运行时和垃圾回收，它能够胜任对性能要求特别高的服务，可以在嵌入式设备上运行，还能轻松和其他语言集成。
- **可靠性**：Rust 丰富的类型系统和所有权模型保证了内存安全和线程安全，让您在编译期就消除各种错误。
- **生产力**：Rust 拥有出色的文档、友好的编译器和清晰的错误提示工具，以及顶尖的集成工具链（Cargo）。

## 2. 环境搭建 (Installation)

推荐使用 `rustup` 安装 Rust。

### Linux / macOS
打开终端运行：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows
访问 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) 下载 `rustup-init.exe` 并安装。

### 验证安装
安装完成后，重启终端，运行：
```bash
rustc --version
cargo --version
```
如果能看到版本号，说明安装成功。

## 3. Hello World

按照编程传统，我们从 Hello World 开始。

### 编写代码
创建一个名为 `main.rs` 的文件，写入以下内容：

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn` 声明一个函数。
- `main` 是程序的入口点。
- `println!` 是一个宏（Macro），用于打印输出。注意它以 `!` 结尾。

### 编译与运行
在终端中运行：
```bash
rustc main.rs
./main  # Windows 下是 .\main.exe
```

## 4. 使用 Cargo
Cargo 是 Rust 的构建系统和包管理器。

### 创建新项目
```bash
cargo new hello_cargo
cd hello_cargo
```

### 目录结构
- `Cargo.toml`: 项目的配置文件（依赖管理）。
- `src/main.rs`: 源代码文件。

### 运行项目
```bash
cargo run
```
这个命令会编译代码，生成可执行文件，并立即执行它。

### 仅编译
```bash
cargo build
```

### 检查代码 (不生成可执行文件，速度更快)
```bash
cargo check
```
