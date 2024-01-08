fn main() {
    // 変数はデフォルトで不変
    let x = 5;

    // 可変にするには、letのあとにmutをつけて宣言する。
    let mut y = 5;
    println!("y = {} (変更前)", y);
    y = 8;
    println!("y = {} (変更後)", y);

    // 定数
    const MAX_POINTS: u32 = 100_000;
    println!("定数: {}", MAX_POINTS);

    // 不変でも、letをつければ再代入が可能！
    // Rustではこれを「シャドーイング」と呼ぶ
    let x = x + 1;

    {
        // スコープの範囲を狭めている？
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);


    // シャドーイングは、型が違くても可能
    let spaces = "   ";  // String型
    let spaces = spaces.len();  // usize型

    // 可変の場合は、型は可変にすることは許されていないため、コンパイルエラーになる。
    // let mut spaces = "   ";
    // spaces = spaces.len();  // expected `&str`, found `usize`
}
