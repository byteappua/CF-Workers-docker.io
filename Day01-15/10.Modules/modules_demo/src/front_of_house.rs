// 声明子模块 hosting
// Rust 会在 front_of_house/hosting.rs 中寻找定义
pub mod hosting;

pub mod serving {
    pub fn take_order() {
        println!("Order taken!");
    }
}
