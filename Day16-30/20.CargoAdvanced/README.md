# Day 20 - Cargo 进阶与 Crates.io 发布

## 1. 发布配置 (Release Profiles)
Cargo 有两个主要的配置：`dev`（开发）和 `release`（发布）。
你可以在 `Cargo.toml` 中自定义它们。

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## 2. 文档注释 (Documentation Comments)
使用 `///` 来编写文档注释。支持 Markdown。
```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo_demo::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
运行 `cargo doc --open` 可以构建并打开 HTML 文档。
文档中的代码块（Examples）也会被作为测试运行：`cargo test`。

### 包含项的注释
使用 `//!` 为包含注释的项（通常是 crate 根或模块）添加文档。

## 3. 发布到 Crates.io
1.  **登录**: `cargo login <token>`
2.  **元数据**: 在 `Cargo.toml` 中添加 `description`, `license` 等。
3.  **发布**: `cargo publish`
    - 注意：一旦发布，版本就永久存在，不能覆盖，只能发新版本。
    - `cargo yank --vers 1.0.1` 可以撤回某个版本（阻止新项目依赖它，但已存在的依赖不受影响）。

## 4. Cargo 工作空间 (Workspaces)
当项目变大时，可以将其拆分为多个 library crate。
工作空间是一组共享同一个 `Cargo.lock` 和输出目录的包。

`Cargo.toml` (根目录):
```toml
[workspace]

members = [
    "adder",
    "add-one",
]
```
