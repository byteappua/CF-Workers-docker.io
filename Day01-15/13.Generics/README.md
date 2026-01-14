# Day 13 - 泛型 (Generics)

泛型 (Generics) 是具体类型或其他属性的抽象替代。我们已经见过泛型了，比如 `Vec<T>` 和 `Option<T>`。

## 1. 在函数定义中使用泛型
当我们要定义一个可以处理多种类型的函数时，可以使用泛型。

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```
注意：这里使用了 Trait Bounds (`PartialOrd + Copy`)，我们将在 Day 14 详细讲解。这里的意思是 T 类型必须能比较大小，并且实现了 Copy Trait。

## 2. 在结构体定义中使用泛型
```rust
struct Point<T> {
    x: T,
    y: T,
}

struct PointMixed<T, U> {
    x: T,
    y: U,
}

let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
```

## 3. 在枚举定义中使用泛型
```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## 4. 在方法定义中使用泛型
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

## 5. 泛型代码的性能
Rust 使用 **单态化 (Monomorphization)** 技术。在编译时，Rust 会将泛型代码转换成针对具体类型的代码。这意味着使用泛型**没有运行时开销**。
