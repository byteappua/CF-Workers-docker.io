struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 1. 结构体实例化
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("User email: {}", user1.email);

    // 结构体更新语法
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // 其他字段使用 user1 的值
    };
    println!("User2 username: {}", user2.username);

    // 2. 元组结构体
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);

    // 3. 方法与关联函数
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle::square(60);

    println!("rect1 is {:?}", rect1); // 需要 #[derive(Debug)]
    println!("The area of the rectangle is {} pixels.", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
