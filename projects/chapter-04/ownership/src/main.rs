fn main() {
    // 変数のスコープ

    // 文字列リテラル(不変)
    // スタックに保管される
    // この文字列の値はプログラムのテキストとしてハードコードされている
    let s = "hello";
    println!("{}", s);

    // String型(可変化することができる)
    // ヒープにメモリを確保する
    // from関数を使用して、 文字列リテラルからString型を生成
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
    println!("{}", s); // これは`hello, world!`と出力する

    // Rustはガベージコレクタを採用しておらず、
    // メモリを所有している変数がスコープを抜けたら、メモリは自動的に返還される

    {
        let s = String::from("hello"); // sはここから有効になる

        // sで作業をする
    }                                  // このスコープはここでおしまい。sはもう有効ではない
    // 変数がスコープを抜ける時、Rustは`drop`という特別な関数を呼んでくれます


    // 値渡し
    // 整数型はコンパイル時に既知のサイズを持つ型
    // なぜなら、整数は既知の固定サイズの単純な値で、これら二つの5という値は、スタックに積まれるから
    // 厳密には、整数型は、Copyトレイトに適合しているから
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // スタックには、String型のポインタ、長さ、 許容量を保管している
    // s1をs2に代入すると、String型のデータがコピーされます。
    // つまり、スタックにあるポインタ、長さ、 許容量をコピーするということです。
    // ポインタが指すヒープ上のデータはコピーしません
    let s1 = String::from("hello");
    let s2 = s1;  // この時点で、s1は無効化される
    // s1はs2にムーブされたと表現する
    println!("{}, world!", s2);  // s1だとエラー

    // スタック上のデータだけでなく、ヒープデータまでもコピーしたい場合
    let s1 = String::from("hello");
    let s2 = s1.clone();  // s1のヒープデータまでのdeep copy
    println!("s1 = {}, s2 = {}", s1, s2);

    // 所有権と関数
    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない
    // println!("{}", s);           // これはエラーになる

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫
    println!("{}", x);              // これはエラーではない

    // 戻り値とスコープ
    let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1にムーブする
    println!("{}", s1);

    let s2 = String::from("hello");     // s2がスコープに入る

    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                        // 戻り値もs3にムーブされる
    // println!("{}", s2);              // これはエラー
    println!("{}", s3);                 // これはエラーではない
}
// ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

// ここで、s3はスコープを抜け、ドロップされる。
// s2もスコープを抜けるが、ムーブされているので、何も起きない。
// s1もスコープを抜け、ドロップされる。


fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。


fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。



fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
                                             // 呼び出した関数にムーブする
    let some_string = String::from("hello"); // some_stringがスコープに入る
    some_string                              // some_stringが返され、呼び出し元関数に
                                             // ムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}



// ヒープデータを管理することが所有権の存在する理由
// ヒープとは？スタックとは？
// https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html


// String型では、コンパイル時には不明な量のメモリをヒープに確保して内容を保持します。つまり
// - メモリは、実行時にOSに要求される。
// - String型を使用し終わったら、OSにこのメモリを返還する方法が必要である。


// RustにはCopyトレイトと呼ばれる特別な注釈があり、 整数のようなスタックに保持される型に対して配置することができます
// 型がCopyトレイトに適合していれば、代入後も古い変数が使用可能になります
