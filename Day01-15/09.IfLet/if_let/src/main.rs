fn main() {
    // 1. 使用 match 处理单个模式 (冗长)
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("(Match) The maximum is configured to be {}", max),
        _ => (),
    }

    // 2. 使用 if let 处理单个模式 (简洁)
    let config_max_2 = Some(10u8);
    if let Some(max) = config_max_2 {
        println!("(If Let) The maximum is configured to be {}", max);
    }

    // 3. if let else
    let coin = Coin::Penny;
    // let coin = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Not a quarter!");
    }
}

enum Coin {
    Penny,
    #[allow(dead_code)]
    Nickel,
    #[allow(dead_code)]
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}
