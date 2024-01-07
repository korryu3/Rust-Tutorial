
use std::io;  // 標準ライブラリ(std)からioモジュールをスコープに入れる
// このセットはprelude（プレリュード）と呼ばれ、標準ライブラリのドキュメントでその中のすべてを見ることができます

fn main() {
    println!("Guess the number!");

    println!("Please input your guess!");

    let mut guess = String::new();  // new()は、String型の関連関数

    // std::io::stdin()とすれば、use std::ioをしなくとも呼び出せる
    io::stdin()  // ioモジュールに紐付けられた関数を呼び出す。
        .read_line( &mut guess ); // 変数のように参照も不変だから、&guessではない。
        // ↑入力した値だけでなく、Result型(列挙型(enum))が帰ってくる
        .expect("Failed to read line"); // Result型のメソッドを呼び出す
    // ↑ .method_name()構文を使い、改行と空白を使って、長い行になることを防ぐ

    println!("You guessed: {}", guess);
}


