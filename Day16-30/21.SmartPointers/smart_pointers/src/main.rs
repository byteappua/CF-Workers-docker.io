use std::ops::Deref;
use std::rc::Rc;

// 1. 递归类型示例
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

// 2. 自定义智能指针实现 Deref
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 3. 实现 Drop
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // Box
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);

    // Deref
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 实际上运行的是 *(y.deref())
    println!("Deref worked!");

    // Deref Coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // &MyBox<String> -> &String -> &str

    // Drop
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // 提前 Drop
    let e = CustomSmartPointer { data: String::from("early drop") };
    drop(e);
    println!("CustomSmartPointer dropped before end of main.");

    // Rc<T>
    let a = Rc::new(Cons(5, Box::new(Cons(10, Box::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // Rc List
    // 注意：前面的 List 定义使用了 Box，为了演示 Rc，我们需要一个新的 List 定义或者修改上面的。
    // 为了简单，我们这里只演示 Rc 的计数功能，不重新定义 List。

    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
