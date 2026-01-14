fn main() {
    // 1. 不可变引用
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 借用 s1
    println!("The length of '{}' is {}.", s1, len);

    // 2. 可变引用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("Changed string: {}", s2);

    // 3. 借用规则演示
    let mut s3 = String::from("hello");

    let r1 = &s3;
    let r2 = &s3;
    // let r3 = &mut s3; // 编译错误：cannot borrow `s3` as mutable because it is also borrowed as immutable

    println!("r1: {}, r2: {}", r1, r2);
    // r1 和 r2 的作用域在这里结束（因为后续没有使用了）

    let r3 = &mut s3; // 现在可以了
    r3.push_str(" world");
    println!("r3: {}", r3);

    // 4. 悬垂引用 (Dangling References)
    // let reference_to_nothing = dangle(); // 编译错误
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s // 返回 s 的引用，但 s 在函数结束时会被 drop，引用将指向无效内存
}
*/
