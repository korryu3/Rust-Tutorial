// 長方形の面積を求めるプログラムを書こう！
// 単一の変数から作り、リファクタリングとして構造体を使用していく

fn main() {
    // 愚直に変数に代入する
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    // タプルを使って、データを一つにまとめる
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    let rect2 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    // 構造体のインスタンスをprintlnで見たい！
    // println!("rect2 is {}", rect2);  // これはエラー
    // printlnは様々な方法で整形して出力する。標準は`std::fmt::Display`
    // {:?}とすれば、Debug(トレイト)という出力整形になる￥
    println!("rect2 is {:?}", rect2)  // {:#?}にすると、もっときれいな形に整形される

}


// widthとheightは関連性があるのに、この関数だと引数は二つになってしまっている
fn area1(width: u32, height: u32) -> u32 {
    width * height
}

// タプルを使って、引数を一つにまとめよう！
fn area2(dimensions: (u32, u32)) -> u32 {
    // さっきよりはマシだけど、これだと明確性を失っている。
    // 計算が不明瞭. 他人がこれを見たらぞっとするだろう
    dimensions.0 * dimensions.1
}

// 構造体を使って、より意味付けする
#[derive(Debug)]  // Debugトレイトを使えるようにする
struct Rectangle {
    width: u32,
    height: u32
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
