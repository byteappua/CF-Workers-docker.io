# Day 15 - 生命周期 (Lifetimes)

生命周期是 Rust 中最独特的泛型。它们的主要目标是避免**悬垂引用**。Rust 编译器有一个借用检查器 (Borrow Checker)，它比较作用域来确保所有的借用都是有效的。

## 1. 为什么需要生命周期？

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    } // x 离开作用域，被销毁
    println!("r: {}", r); // r 引用了无效的内存
}
```
上面的代码会编译失败，因为 `r` 的生命周期比 `x` 长。

## 2. 函数中的生命周期注解
当函数返回一个引用时，Rust 需要知道这个引用来自哪里，以及它能活多久。如果返回的引用依赖于输入的引用，我们就需要生命周期注解。

生命周期注解不会改变任何引用的实际生命周期，它只是向借用检查器描述了多个引用生命周期之间的关系。
语法：`'a` (通常使用短小的名字，如 `'a`, `'b`)。

```rust
// 错误写法
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// 正确写法
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
这告诉 Rust：`longest` 函数接受两个参数，它们的生命周期至少都是 `'a`。返回值的生命周期也是 `'a`，也就是取 `x` 和 `y` 中生命周期较短的那个。

## 3. 结构体定义中的生命周期注解
如果结构体中包含引用，就需要为每个引用添加生命周期注解。

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```
这意味着 `ImportantExcerpt` 的实例不能比其字段 `part` 引用的字符串活得更久。

## 4. 静态生命周期 ('static)
`'static` 是一个特殊的生命周期，它存活于整个程序的运行期间。所有的字符串字面值都拥有 `'static` 生命周期。
```rust
let s: &'static str = "I have a static lifetime.";
```

## 5. 生命周期省略规则 (Lifetime Elision Rules)
在某些常见情况下，Rust 允许我们省略生命周期注解。编译器有三条规则来推断生命周期：
1.  每个引用参数都有其自己的生命周期参数。
2.  如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数。
3.  如果方法有多个输入生命周期参数，但其中一个是 `&self` 或 `&mut self`，那么 `self` 的生命周期被赋给所有输出生命周期参数。
