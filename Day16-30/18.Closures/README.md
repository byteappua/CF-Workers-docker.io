# Day 18 - 闭包 (Closures)

闭包是可以保存进变量或作为参数传递给其他函数的匿名函数。不同于函数，闭包可以捕获其定义时所在作用域中的值。

## 1. 语法
```rust
let plus_one = |x: i32| x + 1;
assert_eq!(2, plus_one(1));
```
闭包定义的语法类似于管道符号 `|args| body`。

## 2. 类型推断
闭包通常不需要像函数那样标注参数和返回值的类型。编译器会根据使用情况推断。
```rust
let example_closure = |x| x;
let s = example_closure(String::from("hello"));
// let n = example_closure(5); // 错误！编译器已经推断 x 是 String 类型
```

## 3. 捕获环境
闭包可以通过三种方式捕获环境，这对应于函数的三种获取参数的方式：获取所有权、可变借用、不可变借用。这也编码在三个 Fn Trait 中：

1.  `FnOnce`: 闭包可以从周围作用域中获取变量的所有权 (`move`)。这表示闭包只能被调用一次。
2.  `FnMut`: 闭包可变地借用值。
3.  `Fn`: 闭包不可变地借用值。

```rust
let x = 4;
let equal_to_x = |z| z == x; // 捕获了 x (不可变借用)
let y = 4;
assert!(equal_to_x(y));
```

### move 关键字
强制闭包获取其使用环境值的所有权。
```rust
let x = vec![1, 2, 3];
let equal_to_x = move |z| z == x;
// println!("can't use x here: {:?}", x); // 错误，x 已经被 move 进闭包
```

## 4. 最佳实践
当你把闭包作为参数传递时，几乎总是使用泛型和 Fn Trait Bounds。
```rust
fn call_with_one<F>(some_closure: F) -> i32
where F: Fn(i32) -> i32 {
    some_closure(1)
}
```
