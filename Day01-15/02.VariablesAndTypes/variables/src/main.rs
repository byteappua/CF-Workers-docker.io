fn main() {
    // 1. 不可变变量
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // 编译错误：cannot assign twice to immutable variable `x`

    // 2. 可变变量
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is now: {}", y);

    // 3. 常量
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    // 4. Shadowing (隐藏/遮蔽)
    let z = 5;
    let z = z + 1; // z = 6
    {
        let z = z * 2; // inner z = 12
        println!("The value of z in the inner scope is: {}", z);
    }
    println!("The value of z is: {}", z); // outer z = 6

    // 5. 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_a, b, _c) = tup;
    println!("The value of b is: {}", b);

    // 6. 数组
    let arr = [1, 2, 3, 4, 5];
    println!("The first element is: {}", arr[0]);
}
