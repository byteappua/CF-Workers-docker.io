fn main() {
    // 1. 字符串切片
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("s: {}", s);
    println!("slice 1: {}", hello);
    println!("slice 2: {}", world);

    // 2. 字符串字面值
    let literal = "Hello literal"; // 类型是 &str
    println!("Literal: {}", literal);

    // 3. 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // 包含索引 1 和 2 的元素
    println!("Array slice: {:?}", slice);

    // 4. 使用切片解决问题
    let mut s2 = String::from("hello world");
    let word = first_word(&s2);

    // s2.clear(); // 错误！s2 被 word 借用（不可变），不能修改

    println!("The first word is: {}", word);

    s2.clear(); // word 不再使用，现在可以修改了
}

fn first_word(s: &str) -> &str { // 参数类型最好写成 &str，这样既可以传 String 也可以传字面值
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
