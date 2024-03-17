// mainはエントリーポイント  ※プログラム実行時に最初に走る関数のこと
fn main() {
    println!("Hello, world!");

    another_func(100);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;  // 文
        x + 1  // 式
        // 式は終端にセミコロンを含まない。
        // 付けたら文になってしまう。
        // 文は値を返さない。 これら大事
    };  // これは文？それとも式? ブロックだから式なのか？
    // このセミコロンは、letの終わりを指すセミコロンなのかな？

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

}

fn five() -> i32 {
    // 関数定義自体は文だが、{}で囲われた中がブロックと考えると、
    // 関数の戻り値は、関数本体ブロックの最後の式の値と同義
    5
    // 5;  // これはエラーになる
    // 文は値を返さない。このことは、 ()、つまり空のタプルとして表現されている。
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1;  // これはエラーになる
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
// 文の内部に式があるイメージ？

/* 公式ドキュメントから
式は何かに評価され、これからあなたが書くRustコードの多くを構成します。
簡単な数学演算(5 + 6など)を思い浮かべましょう。
この例は、値11に評価される式です。
式は文の一部になりえます: リスト3-1において、let y = 6という文の6は値6に評価される式です。
関数呼び出しも式です。
マクロ呼び出しも式です。
新しいスコープを作る際に使用するブロック({})も式です:
*/
