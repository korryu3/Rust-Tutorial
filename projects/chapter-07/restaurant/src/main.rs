// ファイル名を指定すると、そのファイル内のpubモジュールを読み込む
mod front_of_house;

pub use crate::front_of_house::hosting;

fn main() {
    eat_at_restaurant();
    println!("Hello, world!");
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
