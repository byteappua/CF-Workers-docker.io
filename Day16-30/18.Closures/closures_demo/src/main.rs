use std::thread;
use std::time::Duration;

// 模拟一个耗时的计算
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(1));
    intensity
}

// 缓存器 struct (Memoization)
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    // 1. 基础闭包
    let plus_one = |x: i32| x + 1;
    println!("1 + 1 = {}", plus_one(1));

    // 2. 捕获环境
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
    println!("y equals x? {}", equal_to_x(y));

    // 3. Move 闭包
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // move
    let list2 = vec![1, 2, 3];
    println!("Before defining move closure: {:?}", list2);
    thread::spawn(move || {
        println!("From thread: {:?}", list2);
    }).join().unwrap();
    // println!("After thread: {:?}", list2); // Error: value moved

    // 4. 模拟延迟计算 (Lazy Evaluation)
    let mut cacher = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    // 第一次调用，会执行 calculation
    println!("Value: {}", cacher.value(10));
    // 第二次调用，直接返回缓存值
    println!("Value (cached): {}", cacher.value(10));
}
