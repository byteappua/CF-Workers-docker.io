use std::cell::RefCell;
use std::rc::Rc;

// 1. 定义 Messenger Trait
pub trait Messenger {
    fn send(&self, msg: &str);
}

// 2. 模拟需要内部可变性的场景
// 假设我们需要测试一个 LimitTracker，它依赖 Messenger 发送通知
// 但我们不想真的发邮件，而是想在 MockMessenger 中记录发了什么

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // self 是 &self (不可变)，但我们需要修改 sent_messages
        // borrow_mut() 获取内部值的可变引用
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

fn main() {
    // 1. RefCell 演示
    println!("--- RefCell Demo ---");
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);
    // 验证是否发送了消息
    // borrow() 获取不可变引用
    println!("Messages sent: {:?}", mock_messenger.sent_messages.borrow());
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

    // 2. Rc + RefCell 演示
    println!("\n--- Rc + RefCell Demo ---");
    let value = Rc::new(RefCell::new(5));

    let a = Rc::clone(&value);
    let b = Rc::clone(&value);

    // 修改 a 指向的值
    *value.borrow_mut() += 10;

    println!("value = {:?}", value);
    println!("a = {:?}", a);
    println!("b = {:?}", b);

    // 运行时借用检查失败演示 (会导致 panic)
    // let mut one_borrow = value.borrow_mut();
    // let mut two_borrow = value.borrow_mut(); // Panic here
}
