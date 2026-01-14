# Day 06 - 切片 (Slices)

切片 (Slice) 允许你引用集合中一段连续的元素序列，而不需要引用整个集合。切片是一种引用，所以它没有所有权。

## 1. 字符串切片 (String Slices)
字符串切片是指向 `String` 中一部分内容的引用。

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

println!("{}", hello); // hello
println!("{}", world); // world
```

### 范围语法 (Range Syntax)
- `[0..2]` 等同于 `[..2]`
- `[3..len]` 等同于 `[3..]`
- `[0..len]` 等同于 `[..]`

### 字符串字面值就是切片
```rust
let s = "Hello, world!"; // s 的类型是 &str
```
这里 `s` 是一个指向二进制程序特定位置的切片。这也是字符串字面值不可变的原因。

## 2. 数组切片 (Array Slices)
正如我们可以引用字符串的一部分一样，我们也可以引用数组的一部分。

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

## 3. 为什么需要切片？
切片解决了“引用失效”的问题。如果仅仅保存索引，当原数据改变时，索引可能变得无效。但切片绑定到了原数据上，借用检查器会确保切片有效期内，原数据不被修改（如果是不可变切片）。

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
