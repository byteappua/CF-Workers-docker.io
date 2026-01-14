# Day 24 - 并发编程基础 (Concurrency)

Rust 的并发模型旨在帮助你编写安全、高效的并发代码。Rust 的所有权和类型系统在编译时就能捕获许多并发错误。

## 1. 线程 (Threads)
Rust 使用 **1:1** 线程模型，即程序中的每个线程都映射到一个操作系统线程。
使用 `std::thread::spawn` 创建新线程。

```rust
use std::thread;
use std::time::Duration;

thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
});
```
当主线程结束时，所有派生线程都会被强制终止。

## 2. JoinHandle
`thread::spawn` 返回一个 `JoinHandle`。调用 `join()` 方法会阻塞当前线程，直到对应的派生线程结束。

```rust
let handle = thread::spawn(|| { ... });
handle.join().unwrap();
```

## 3. 线程与 move 闭包
为了在线程中使用外部数据，通常需要使用 `move` 关键字将数据的所有权移入闭包。

```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});
// println!("{:?}", v); // 错误，v 已被移动
handle.join().unwrap();
```
