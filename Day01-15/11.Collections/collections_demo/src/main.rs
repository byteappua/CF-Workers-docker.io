use std::collections::HashMap;

fn main() {
    // 1. Vector
    println!("--- Vector ---");
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);

    // 索引访问
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // get 方法访问
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 遍历并修改
    for i in &mut v {
        *i += 50;
    }
    println!("Modified vector: {:?}", v);

    // 2. String
    println!("\n--- String ---");
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 转移所有权，s2 借用
    let s3 = s1 + &s2;
    println!("s3: {}", s3);
    // println!("s1: {}", s1); // Error: value borrowed here after move

    // 3. HashMap
    println!("\n--- HashMap ---");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue team score: {}", score);

    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 统计单词出现次数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word count: {:?}", map);
}
