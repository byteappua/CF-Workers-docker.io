# Day 23 - 引用循环与内存泄漏 (Reference Cycles)

Rust 的内存安全性保证使其很难制造内存泄漏，但不是不可能。如果使用 `Rc<T>` 和 `RefCell<T>` 创造出引用循环，每一项的引用计数永远不会变为 0，值也就永远不会被丢弃。

## 1. 制造引用循环
```rust
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
```
如果我们将 A 指向 B，B 指向 A，它们的 `strong_count` 都会一直保持 > 0。

## 2. Weak<T> (弱引用)
为了解决引用循环，可以将其中一个链接变为 **弱引用 (Weak Reference)**。
弱引用不会增加 `strong_count`，而是增加 `weak_count`。`weak_count` 不用为 0 就能使对象被销毁。

### 升级 Weak 引用
因为 `Weak<T>` 引用的值可能已经被销毁了，为了使用它，必须调用 `upgrade` 方法，这会返回 `Option<Rc<T>>`。

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // 指向父节点，使用 Weak 防止循环引用
    children: RefCell<Vec<Rc<Node>>>,
}

let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
});

let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
});

// 让 leaf 指向 branch
*leaf.parent.borrow_mut() = Rc::downgrade(&branch);
```

### Strong vs Weak
- **Strong Reference**: 所有权关系。只要存在，数据就在。
- **Weak Reference**: 观察者关系。不保证数据还在。
