# Day 16 - 自动化测试 (Automated Tests)

Rust 拥有内置的测试框架。测试是一个函数，用于验证非测试代码的功能是否和预期一致。

## 1. 单元测试 (Unit Tests)
单元测试位于 `src` 目录中，与被测试的代码在一起。通常会在每个文件中创建一个 `tests` 模块，并使用 `#[cfg(test)]` 进行标注。

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```
运行测试：`cargo test`

### 常用宏
- `assert!(expression)`: 验证布尔值。
- `assert_eq!(left, right)`: 验证相等。
- `assert_ne!(left, right)`: 验证不相等。

### 自定义错误信息
```rust
assert!(result.contains("Carol"), "Result did not contain name, value was `{}`", result);
```

### 检查 Panic
使用 `#[should_panic]` 属性。
```rust
#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}
```

## 2. 集成测试 (Integration Tests)
集成测试位于项目根目录下的 `tests` 目录中（与 `src` 同级）。它们将库视为外部 crate。

`tests/integration_test.rs`:
```rust
use tests_demo; // 引入你的库

#[test]
fn it_adds_two() {
    assert_eq!(4, tests_demo::add_two(2));
}
```

## 3. 控制测试运行
- `cargo test --help`
- `cargo test -- --test-threads=1`: 单线程运行（避免并发干扰）。
- `cargo test -- --show-output`: 显示成功测试的输出（默认被捕获）。
- `cargo test one_hundred`: 运行名称包含 "one_hundred" 的测试。
