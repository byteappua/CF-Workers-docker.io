# Day 10 - 模块系统 (Module System)

Rust 的模块系统包括：
- **包 (Packages)**: Cargo 的一个功能，允许你构建、测试和分享 crate。
- **Crates**: 一个模块的树形结构，它产生一个库或可执行文件。
- **模块 (Modules)** 和 **use**: 允许你控制作用域和路径的私有性。
- **路径 (Paths)**: 为结构体、函数或模块等项命名的方式。

## 1. 模块定义 (Modules)
使用 `mod` 关键字。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

## 2. 路径 (Paths)
- **绝对路径**: 从 crate 根开始，以 crate 名或 `crate` 字面值开头。
- **相对路径**: 从当前模块开始，以 `self`、`super` 或当前模块的标识符开头。

```rust
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

## 3. 私有性边界 (Privacy Boundary)
- 默认情况下，Rust 中的一切都是**私有的**。
- 父模块无法访问子模块中的私有条目，但子模块可以访问祖先模块中的条目。
- 使用 `pub` 关键字将条目变为公有。

## 4. use 关键字
使用 `use` 将路径引入作用域，就像创建了软连接一样。

```rust
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

## 5. 将模块拆分到不同文件
- `src/lib.rs`: crate 根。
- `src/front_of_house.rs`: 定义 `front_of_house` 模块。
- `src/front_of_house/hosting.rs`: 定义 `hosting` 子模块。

`src/lib.rs`:
```rust
mod front_of_house; // 声明模块，Rust 会在同名文件中查找定义

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
