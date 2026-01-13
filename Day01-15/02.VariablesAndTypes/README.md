# Day 02 - 变量与数据类型

## 1. 变量 (Variables)与可变性 (Mutability)

在 Rust 中，变量默认是**不可变**的 (immutable)。这是 Rust 为了提供并发安全和避免 bug 而做出的设计选择。

### 声明变量
使用 `let` 关键字声明变量。

```rust
let x = 5;
// x = 6; // 报错！因为 x 默认不可变
```

### 可变变量
如果需要修改变量的值，必须使用 `mut` 关键字。

```rust
let mut y = 5;
y = 6; // 合法
```

### 常量 (Constants)
使用 `const` 关键字声明，必须注明类型，且只能绑定到常量表达式。
```rust
const MAX_POINTS: u32 = 100_000;
```

### 隐藏 (Shadowing)
你可以声明一个同名的新变量，新变量会“遮蔽”旧变量。这通常用于类型转换。
```rust
let x = 5;
let x = x + 1; // x 现在是 6
let x = "six"; // x 现在是字符串切片，类型改变了
```

## 2. 数据类型 (Data Types)

Rust 是静态类型语言，但在大多数情况下，编译器可以根据值推断出类型。

### 标量类型 (Scalar Types)
代表一个单独的值。

1.  **整型 (Integers)**:
    - `i8`, `u8` (8-bit)
    - `i32`, `u32` (32-bit, 默认)
    - `i64`, `u64` (64-bit)
    - `isize`, `usize` (取决于架构，32位或是64位)

2.  **浮点型 (Floating-Point)**:
    - `f32`
    - `f64` (默认，精度更高)

3.  **布尔型 (Boolean)**:
    - `bool` (值为 `true` 或 `false`)

4.  **字符型 (Character)**:
    - `char`: 代表 Unicode 标量值 (4 bytes)，用单引号 `'` 包围。

### 复合类型 (Compound Types)
可以将多个值组合成一个类型。

1.  **元组 (Tuple)**:
    - 长度固定，可以包含不同类型。
    ```rust
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 解构
    println!("y is: {}", tup.1); // 使用索引访问
    ```

2.  **数组 (Array)**:
    - 长度固定，必须包含**相同**类型。数据存放在栈(stack)上。
    ```rust
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    ```
