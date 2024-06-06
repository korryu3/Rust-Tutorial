fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // wordの中身は、値5になる

    s.clear(); // Stringを空にする。つまり、""と等しくする

    // wordはまだ値5を保持しているが、もうこの値を正しい意味で使用できる文字列は存在しない。
    // wordは今や完全に無効なのだ！

    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error! （エラー！）不変な借用じゃなきゃダメ！ (文字列を""に変更するため)

    println!("the first word is: {}", word);


    let s = String::from("hello world");

    let hello = &s[0..5];  // String型の一部の参照
    let world = &s[6..11];  // 添字6へのポインターと、長さ5の情報を持つスライス。(lenは、ending_index(11) から starting_index(5) を引いたもの)
    // つまり、範囲としては、添字6, 7, 8, 9, 10の文字列を得る
    println!("slice: {}", world);

    let slice = &s[0..2];
    let slice = &s[..2];  // 0..2を意味する。
    println!("slice: {}", slice);


    let len = s.len();

    let slice = &s[3..len];  // 3から終点(lenは11)まで
    let slice = &s[3..];  // 上と等価
    println!("slice: {}", slice);

    let slice = &s[0..len];  // 最初から最後まで
    let slice = &s[..];  // 上と等価。文字列全体を示す
    println!("slice: {}", slice);


    let my_string = String::from("hello world");

    // first_wordは`String`のスライスに対して機能する
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_wordは文字列リテラルのスライスに対して機能する
    let word = first_word(&my_string_literal[..]);

    // 文字列リテラルは「それ自体すでに文字列スライスなので」、
    // スライス記法なしでも機能するのだ！
    let word = first_word(my_string_literal);

    // 配列の値をスライスするときも文字列同様
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];  // &[i32]型
}

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {  // 文字列スライスも引数に入れたい！String型は、s[..]のようにすれば良い！
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {  // バイトリテラル表記 (バイトの32番が半角スペース)
            return &s[..i];
        }
    }

    &s[..]  // s[..i]はsの一部を"借用"しているだけだから、参照で返さないと、sの所有権がムーブされてしまう(多分)
    // &strの疑問解決
    // [Rust - strが引数/戻り値で使えない理由について](https://zenn.dev/philomagi/articles/rust_why_cant_use_str_as_both_argument_and_return)
    // 関数の引数として渡された値はスタックへ乗せられる
    // strだけだと任意長なので、参照にすることで、strへのポインタ(固定長)を乗せている
    // 関数の戻り値もやはりスタックへ乗せられる
}


// Rustでは、".."は範囲記法

// 所有権、借用、スライスの概念は、Rustプログラムにおいて、コンパイル時にメモリ安全性を保証します！！！！！
