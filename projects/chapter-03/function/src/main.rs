// mainはエントリーポイント  ※プログラム実行時に最初に走る関数のこと
fn main() {
    println!("Hello, world!");

    another_func(100);
    print_labeled_measurement(5, 'h');
}

// Rustはスネークケース
// 仮引数は必ず型を宣言しなければならない。
fn another_func(x: i32) {
    println!("The value of x is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}


// 関数はどこで定義してもok
// mainの上でも下でも可能


// parameter: 仮引数
// argument: 実引数
// 実引数: 実際に引数に入れる値のこと


// Rustは式指向言語
// 文とは、なんらかの動作をして値を返さない命令
// 式は結果値に評価されます。
