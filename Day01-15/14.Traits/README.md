# Day 14 - Trait (特征)

Trait (特征) 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。Trait 类似于其他语言中的 **接口 (Interfaces)**，但有些不同。

## 1. 定义 Trait
```rust
pub trait Summary {
    fn summarize(&self) -> String;

    // 默认实现
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}
```

## 2. 为类型实现 Trait
```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

## 3. Trait 作为参数
可以使用 `impl Trait` 语法。

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

### Trait Bound 语法
这是一种更长的形式，对于复杂的泛型很有用。
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### 指定多个 Trait Bound
```rust
pub fn notify(item: &(impl Summary + Display)) { ... }
pub fn notify<T: Summary + Display>(item: &T) { ... }
```

### where 从句
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ ... }
```

## 4. 返回实现了 Trait 的类型
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```
注意：你只能返回一种可能的类型。如果函数可能返回 Tweet 也可能返回 NewsArticle，这种写法是行不通的（需要 Trait Objects，将在进阶部分讲解）。
