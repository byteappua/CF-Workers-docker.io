fn main() {
    // 1. Move (移动)
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // Error: value borrowed here after move
    println!("s2: {}", s2);

    // 2. Clone (克隆)
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);

    // 3. Copy (复制) - 栈上数据
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // 4. 所有权与函数
    let s5 = String::from("function");
    takes_ownership(s5);
    // println!("{}", s5); // Error

    let s6 = gives_ownership();
    println!("s6: {}", s6);

    let s7 = String::from("takes and gives");
    let s8 = takes_and_gives_back(s7);
    println!("s8: {}", s8);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
} // some_string dropped here

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
