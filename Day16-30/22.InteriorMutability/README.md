# Day 22 - 内部可变性 (Interior Mutability)

内部可变性是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的。为了做到这一点，该模式在数据结构中使用 `unsafe` 代码来模糊 Rust 通常的可变性和借用规则。

## 1. `RefCell<T>`
不同于 `Rc<T>`，`RefCell<T>` 代表其数据的唯一所有权。
但是，它将在**运行时**而不是编译时检查借用规则。

- **编译时检查**: 大多数 Rust 分析。如果不符合规则，编译失败。
- **运行时检查**: `RefCell<T>`。如果不符合规则，程序 panic。

### 为什么需要？
当你确信你的代码遵守借用规则，但编译器无法理解和保证时。

## 2. 借用规则回顾
1.  在任意给定时间，你只能拥有如下中的一个：
    - 一个可变引用。
    - 任意数量的不可变引用。
2.  引用必须总是有效的。

对于 `RefCell<T>`：
- `borrow()` 方法返回 `Ref<T>`（类似 `&T`）。
- `borrow_mut()` 方法返回 `RefMut<T>`（类似 `&mut T`）。
- `RefCell` 会记录当前有多少个 `Ref` 和 `RefMut`。如果尝试在有 `Ref` 时创建 `RefMut`，或者创建两个 `RefMut`，程序会 panic。

## 3. 内部可变性模式的应用：Mock 对象
有时我们需要在不可变引用的上下文中修改数据。例如，实现一个 Trait，其方法签名是 `&self`，但我们想在测试中记录调用次数。

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

// Mock 对象
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // self 是不可变的，但我们可以借用 sent_messages 为可变
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}
```

## 4. 结合 `Rc` 和 `RefCell`
拥有多重所有权的可变数据。

```rust
let value = Rc::new(RefCell::new(5));

let a = Rc::clone(&value);
let b = Rc::clone(&value);

*value.borrow_mut() += 10;
```
