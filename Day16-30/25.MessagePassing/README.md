# Day 25 - 消息传递 (Message Passing)

为了确保并发安全，Rust 采用了 Go 语言社区流行的一句口号：
> 不要通过共享内存来通讯；而是通过通讯来共享内存。

Rust 标准库提供了通道 (Channel) 实现，包含发送端 (Transmitter) 和接收端 (Receiver)。

## 1. 创建通道
使用 `std::sync::mpsc::channel`。`mpsc` 代表 **多个生产者，单个消费者 (Multiple Producer, Single Consumer)**。

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
```

## 2. 发送数据
```rust
use std::thread;

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    // val 已经被移动到通道中，这里不能再使用
});
```

## 3. 接收数据
`rx` 有两个主要方法：
- `recv()`: 阻塞直到收到一个值。
- `try_recv()`: 不阻塞，立即返回 `Result`。

```rust
let received = rx.recv().unwrap();
println!("Got: {}", received);
```

## 4. 迭代器接收
接收端 `rx` 可以被当作迭代器使用。当通道关闭且没有数据时，迭代结束。

```rust
for received in rx {
    println!("Got: {}", received);
}
```

## 5. 克隆发送端
虽然是单个消费者，但可以通过克隆发送端来拥有多个生产者。

```rust
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();

thread::spawn(move || { tx1.send(...).unwrap(); });
thread::spawn(move || { tx.send(...).unwrap(); });
```
