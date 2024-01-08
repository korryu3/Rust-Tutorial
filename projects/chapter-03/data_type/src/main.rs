
#![allow(unused)]
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");    // 数字ではありません！

    // これはエラーが出る。
    // let guess = "42".parse().expect("Not a number!");
}

