# Day 07 - 结构体 (Structs)

结构体让你可以创建出自定义的数据类型，将多个相关的值组合在一起。

## 1. 定义与实例化结构体
使用 `struct` 关键字。

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);
}
```

### 字段初始化简写
如果变量名和字段名相同，可以省略字段名。
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email, // 等同于 email: email
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

## 2. 元组结构体 (Tuple Structs)
没有命名字段的结构体，只有字段类型。
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

## 3. 方法 (Methods)
方法与函数类似，但它们是在结构体（或枚举、Trait对象）的上下文中定义的。
第一个参数总是 `self`，代表调用该方法的实例。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数 (Associated Functions): 不以 self 为第一参数
    // 通常用于构造函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} pixels.", rect1.area());

    let sq = Rectangle::square(3);
}
```

## 4. 打印结构体
默认情况下结构体不能被打印。需要加上 `#[derive(Debug)]` 属性。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1); // 或者 {:#?} 进行格式化打印
}
```
