use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 1. 基础通道
    println!("--- Basic Channel ---");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("Sender: sending '{}'", val);
        tx.send(val).unwrap();
        // val moved
    });

    let received = rx.recv().unwrap();
    println!("Receiver: Got: {}", received);

    // 2. 发送多个值并遍历
    println!("\n--- Multiple Values ---");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // 3. 克隆发送者 (Multiple Producers)
    println!("\n--- Multiple Producers ---");
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread"),
            String::from("1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
