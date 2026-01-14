struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // 1. 基础迭代器
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // 2. 消费适配器 (Consuming Adaptors)
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("Total sum: {}", total);

    // 3. 迭代器适配器 (Iterator Adaptors)
    let v1: Vec<i32> = vec![1, 2, 3];
    // map 是惰性的，必须随后调用消费方法如 collect
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("Map result: {:?}", v2);

    // filter
    let shoes = vec![
        String::from("sandal"),
        String::from("boot"),
        String::from("sneaker"),
    ];
    let my_size = 10;
    // 假设我们有一个 filter 逻辑，这里简化演示
    let s_shoes: Vec<_> = shoes.into_iter().filter(|s| s.starts_with("s")).collect();
    println!("Shoes starting with 's': {:?}", s_shoes);

    // 4. 自定义迭代器
    println!("Custom Counter:");
    let mut counter = Counter::new();
    while let Some(v) = counter.next() {
        println!("{}", v);
    }

    // 使用 Iterator trait 提供的其他方法
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    // 1*2=2, 2*3=6, 3*4=12, 4*5=20
    // 6 + 12 = 18
    println!("Complex iterator chain sum: {}", sum);
}
