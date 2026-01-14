# Day 21 - 智能指针 (Smart Pointers)

指针是一个包含内存地址的变量。Rust 中最常见的指针是引用 (`&`)。
智能指针是一类数据结构，它们表现得像指针，但拥有额外的元数据和功能。最显著的区别是，智能指针通常**拥有**它们指向的数据。

## 1. `Box<T>`
`Box<T>` 是最简单的智能指针，允许你将数据存储在**堆 (Heap)** 上，而不是栈上。

### 适用场景
- 当类型的大小在编译时无法确定时（如递归类型）。
- 当你有大量数据并希望在确保数据不被复制的情况下转移所有权时。

```rust
let b = Box::new(5);
println!("b = {}", b);
```

### 递归类型
```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

## 2. `Deref` Trait
`Deref` trait 允许我们重载解引用运算符 `*`。实现了 `Deref` 的智能指针可以像引用一样被处理。

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```
**Deref 强制转换 (Deref Coercions)**: Rust 可以自动将实现了 Deref 的类型的引用转换为其 Target 类型的引用。例如，`&MyBox<String>` 可以自动转为 `&String`，进而转为 `&str`。

## 3. `Drop` Trait
`Drop` trait 允许我们在值离开作用域时执行代码。通常用于释放资源（如文件、网络连接）。

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```
Rust 会自动调用 `drop`。你不能显式调用 `c.drop()`，但可以调用 `std::mem::drop(c)` 来提前释放。

## 4. `Rc<T>` (Reference Counted)
`Rc<T>` 引用计数智能指针，允许一个值有多个所有者。
- 仅用于单线程场景。
- 通过 `Rc::clone(&a)` 增加引用计数（不会深拷贝数据，只增加计数）。
- 当计数为 0 时，数据被清理。

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a); // 计数 +1
```
