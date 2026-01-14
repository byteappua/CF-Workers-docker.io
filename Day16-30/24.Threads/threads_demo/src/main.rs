use std::thread;
use std::time::Duration;

fn main() {
    // 1. 创建线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 2. 等待线程结束
    handle.join().unwrap();
    println!("Spawned thread finished.");

    // 3. Move 闭包
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // v 已经移动到线程中，这里无法再使用
    // println!("I can't access v here: {:?}", v);

    handle.join().unwrap();
}
