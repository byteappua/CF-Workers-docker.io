# Day 09 - if let 控制流

`if let` 语法让你可以以一种不那么冗长的方式处理只匹配一个模式的值而忽略其他模式的情况。

## 1. 为什么需要 if let？
`match` 强制你处理所有可能的情况。如果你只关心一种情况，你需要使用 `_` 分支。

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```
这有点啰嗦。

## 2. if let 语法
`if let` 接受一个模式和一个表达式，如果表达式的值匹配模式，则执行代码块。

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```
这就简洁多了。

## 3. else
`if let` 也可以搭配 `else` 使用。

```rust
let coin = Coin::Penny;
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

## 4. 总结
- `match`: 想处理所有可能的情况时使用。
- `if let`: 只想处理一种匹配情况（可选带 `else` 处理其他情况）时使用。
