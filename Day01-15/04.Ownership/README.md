# Day 04 - 所有权 (Ownership)

所有权是 Rust 最独特的特性，它让 Rust 无需垃圾回收 (GC) 即可保证内存安全。

## 1. 栈 (Stack) 与 堆 (Heap)
- **栈**: 存储固定大小的数据。后进先出 (LIFO)。速度快。
- **堆**: 存储在编译时大小未知或可能变化的数据。通过指针访问。速度较慢。

## 2. 所有权规则
1.  Rust 中的每一个值都有一个被称为其 **所有者 (owner)** 的变量。
2.  值在任一时刻有且只有一个所有者。
3.  当所有者（变量）离开作用域，这个值将被丢弃 (drop)。

## 3. 变量作用域 (Scope)
```rust
{                      // s 在这里无效，它尚未声明
    let s = "hello";   // 从此处起，s 是有效的
    // 使用 s
}                      // 此作用域已结束，s 不再有效
```

## 4. Move (移动) 语义
对于存储在堆上的数据（例如 `String`），当变量赋值给另一个变量时，所有权会发生**移动**。

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 的所有权移动到了 s2

// println!("{}, world!", s1); // 错误！s1 已经无效了
println!("{}, world!", s2); // 正确
```
这避免了二次释放 (double free) 错误。

## 5. Clone (克隆)
如果确实需要深度复制堆上的数据，可以使用 `clone` 方法。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2); // s1 依然有效
```

## 6. Copy (复制)
对于整型等存储在栈上的简单类型，赋值是进行**复制**，旧变量依然有效。
```rust
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y); // 都可以使用
```

## 7. 所有权与函数
- 将值传递给函数类似于赋值语句。会发生**移动**或**复制**。
- 返回值也可以转移所有权。

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // 错误，s 的所有权已经移动到函数里了

    let x = 5;
    makes_copy(x);
    // x 依然有效
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string 离开作用域，drop 被调用，内存被释放

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```
