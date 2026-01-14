# Day 05 - 引用与借用 (References and Borrowing)

所有权规则虽然严格，但如果我们每次使用值都需要转移所有权，会非常麻烦。**引用 (Reference)** 允许我们访问值而不获取其所有权。获取引用作为函数参数称为 **借用 (Borrowing)**。

## 1. 引用 (References)
使用 `&` 符号创建引用。

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传递引用
    println!("The length of '{}' is {}.", s1, len); // s1 仍然有效
}

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // s 离开作用域，但它不拥有值，所以什么也不会发生
```

## 2. 可变引用 (Mutable References)
默认情况下，引用是不可变的。如果我们需要修改借用的值，必须使用 `&mut`。

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## 3. 借用规则 (The Rules of References)
Rust 的借用检查器 (Borrow Checker) 强制执行以下规则，以防止数据竞争 (Data Races)：

1.  **在任意给定时间，你只能拥有如下中的一个**：
    - 一个可变引用 (`&mut T`)。
    - 任意数量的不可变引用 (`&T`)。
2.  引用必须总是有效的 (即不能悬垂，Dangling Reference)。

### 为什么不能同时拥有可变和不可变引用？
如果有人正在读取数据（不可变引用），他们不希望数据在读取过程中被修改（可变引用）。

```rust
let mut s = String::from("hello");

let r1 = &s; // OK
let r2 = &s; // OK
let r3 = &mut s; // 错误！已经有了不可变引用，不能再创建可变引用

println!("{}, {}, and {}", r1, r2, r3);
```

### 作用域的重要性
引用的作用域从声明开始，持续到**最后一次使用**的地方。

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2);
// r1 和 r2 在这里不再被使用

let r3 = &mut s; // OK，因为 r1, r2 已经结束了生命周期
println!("{}", r3);
```
