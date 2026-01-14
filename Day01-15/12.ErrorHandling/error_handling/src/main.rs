use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    // 1. panic!
    // panic!("crash and burn"); // Uncomment to see panic

    // 2. Result with match
    println!("--- Result with match ---");
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    println!("File opened/created successfully: {:?}", f);

    // 3. unwrap / expect
    // let f = File::open("non_existent.txt").unwrap(); // Will panic
    // let f = File::open("non_existent.txt").expect("Failed to open"); // Will panic with msg

    // 4. 传播错误
    println!("\n--- Propagating Errors ---");
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {:?}", e),
    }

    // 清理创建的文件
    std::fs::remove_file("hello.txt").unwrap_or(());
}

fn read_username_from_file() -> Result<String, io::Error> {
    // 如果文件不存在，open 返回 Err，? 运算符会让函数立即返回该 Err
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
