# Day 11 - 常用集合 (Common Collections)

Rust 标准库中包含一系列被称为 **集合 (collections)** 的数据结构。不同于内置的数组和元组，这些集合指向的数据存储在堆上，这意味着数据的大小不需要在编译时确定，并且可以随程序的运行而增长或缩小。

## 1. Vector (`Vec<T>`)
`Vec<T>` 也被称为 vector，是一个可以存储多个值的连续数组。它只能存储相同类型的值。

```rust
// 创建新的 vector
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3]; // 使用宏

// 更新 vector
let mut v = Vec::new();
v.push(5);
v.push(6);

// 读取元素
let third: &i32 = &v[2]; // 索引访问，越界会 panic
let third: Option<&i32> = v.get(2); // get 方法，越界返回 None

// 遍历
for i in &v {
    println!("{}", i);
}
```

## 2. String
Rust 的核心语言层面只有一个字符串类型：字符串切片 `str` (通常以借用形式 `&str` 出现)。
`String` 类型是由标准库提供的，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串。

```rust
// 创建 String
let mut s = String::new();
let s = "initial contents".to_string();
let s = String::from("initial contents");

// 更新 String
let mut s = String::from("foo");
s.push_str("bar");
s.push('l'); // push 单个字符

// 拼接
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 被移动了，不能再使用
```

## 3. HashMap (`HashMap<K, V>`)
`HashMap<K, V>` 存储了键 (Key) 和值 (Value) 的映射关系。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// 插入
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 访问
let team_name = String::from("Blue");
let score = scores.get(&team_name); // 返回 Option<&V>

// 遍历
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// 更新：只在键没有对应值时插入
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
```
