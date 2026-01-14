// 声明模块 front_of_house
// Rust 会在 front_of_house.rs 中寻找定义
mod front_of_house;

// 使用 use 引入路径
// pub use 重新导出
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    println!("--- Eating at restaurant ---");
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 通过 use 引入的路径
    hosting::add_to_waitlist();

    // 访问另一个子模块
    front_of_house::serving::take_order();
}
