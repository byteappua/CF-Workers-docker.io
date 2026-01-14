# Day 19 - 迭代器 (Iterators)

迭代器模式允许你对一个项的序列进行某些处理。迭代器负责遍历序列中的每一项和决定序列何时结束。

## 1. Iterator Trait
所有迭代器都实现了标准库定义的 `Iterator` trait。
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 此处省略了默认实现的方法
}
```

## 2. 使用迭代器
```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter(); // 创建迭代器

for val in v1_iter {
    println!("Got: {}", val);
}
```

## 3. 消费迭代器的方法 (Consuming Adaptors)
调用 `next` 方法的方法被称为消费适配器。
- `sum()`: 对所有项求和。
- `collect()`: 将迭代器转换成集合。

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
let total: i32 = v1_iter.sum(); // v1_iter 被消费了
```

## 4. 产生其他迭代器的方法 (Iterator Adaptors)
允许我们将当前迭代器变为不同类型的迭代器。
- `map()`: 对每个元素应用闭包。
- `filter()`: 根据闭包结果过滤元素。

```rust
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```
注意：迭代器是**惰性 (lazy)** 的。如果你不调用 `collect` 或其他消费方法，`map` 里的闭包永远不会执行。

## 5. 实现 Iterator Trait
我们可以为自定义类型实现 `Iterator`。只需要定义 `Item` 类型关联并实现 `next` 方法。

```rust
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```
