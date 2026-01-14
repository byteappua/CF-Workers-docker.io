# Day 12 - 错误处理 (Error Handling)

Rust 将错误组合成两个主要类别：
1.  **可恢复错误 (Recoverable errors)**: 如文件未找到。通常使用 `Result<T, E>`。
2.  **不可恢复错误 (Unrecoverable errors)**: 如访问越界。通常使用 `panic!`。

## 1. panic!
当 panic 发生时，程序会打印一个错误信息，展开并清理栈数据，然后退出。

```rust
fn main() {
    panic!("crash and burn");
}
```

## 2. Result
`Result` 枚举定义如下：
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

处理 Result 的几种方式：

### match
```rust
use std::fs::File;

let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
};
```

### unwrap 和 expect
- `unwrap`: 如果 Result 是 Ok，返回 Ok 中的值。如果是 Err，调用 panic!。
- `expect`: 类似 unwrap，但允许你指定 panic! 的错误信息。

```rust
let f = File::open("hello.txt").unwrap();
let f = File::open("hello.txt").expect("Failed to open hello.txt");
```

## 3. 传播错误
当编写一个函数时，我们可以将错误返回给调用者，而不是在函数内部处理它。这称为**传播 (propagating)** 错误。

### ? 运算符
`?` 运算符用在返回 Result 的表达式后面。
如果值是 `Ok`，`Ok` 中的值就是表达式的结果，程序继续执行。
如果值是 `Err`，`Err` 将作为整个函数的返回值返回。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```
注意：`?` 只能用于返回类型为 `Result` (或 `Option` / `FromResidual`) 的函数中。
