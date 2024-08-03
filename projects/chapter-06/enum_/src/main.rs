fn main() {
    // enumを学ぶ
    // IPアドレスを保管する場合
    enum IpAddrKind1 {
        V4,
        V6,
    }

    // 各列挙子のインスタンスを作成
    // 二つの変数は同じ型(IpAddrKind1)
    let four = IpAddrKind1::V4;
    let six = IpAddrKind1::V6;
    fn route(ip_type: IpAddrKind1) { }
    route(IpAddrKind1::V4);  // 同じ型だから、こういう風にできる
    route(IpAddrKind1::V6);  // 同じ型だから、こういう風にできる

    // 下記やり方ができるので、このやり方はやらなくてよい！！
    // 構造体に列挙子とそれに紐づく値を結びつける
    struct IpAddr {
        kind: IpAddrKind1,
        address: String,  // ここの型は一定
    }

    let home = IpAddr {
        kind: IpAddrKind1::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind1::V6,
        address: String::from("::1"),
    };


    // enum自体に値を紐づけることで、上記のような構造体を使ったやり方はしなくてよい

    // 列挙子に型をつけて値を紐づけることもできる
    enum IpAddrKind2 {
        V4(String),
        V6(String),
    }

    let home = IpAddrKind2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind2::V6(String::from("::1"));
    // 且つ、構造体だと下記やり方ができないので、使わなくてよい


    // 各列挙子に紐付けるデータの型と量は、異なってもいい
    // 構造体だとこれはできない
    enum IpAddrKind3 {
        V4(u8, u8, u8, u8),  // v4はアドレスを数値として持ちたい
        V6(String),  // v6はアドレスを文字列として持ちたい
    }

    let home = IpAddrKind3::V4(127, 0, 0, 1);
    let loopback = IpAddrKind3::V6(String::from("::1"));


    // 色んな型が結びついている
    enum Message {
        Quit,  // 紐付けられたデータなし
        Move { x: i32, y: i32 },  // 匿名構造体
        Write(String),  // 単独のStringオブジェクト
        ChangeColor(i32, i32, i32),  // 3つのi32
    }

    // enumはstructと同じように、implを使ってメソッドを定義することができる
    impl Message {
        fn call(&self) {
            // method body would be defined here
            // メソッド本体はここに定義される
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Rustにはnullがない代わりに、Optionというenumが存在する
    // Optionは、明示的にスコープに導入する必要がない
    // それはSomeとNoneにもいえる
    // 標準ライブラリ内での実装は下記
    // enum Option<T> {
    //     Some(T),  // Tはジェネリック型引数であり、Someがあらゆるデータ型を持つことができることを意味する
    //     None,
    // }

    // SomeもNoneも、あくまでOptionの列挙子
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;  // Noneを使ったら、コンパイラにOption<T>が何を取るか教えてあげないといけない
    // i32 + Option<i32>の計算はできない
}


// enum列挙子内には、いかなる種類のデータでも格納できる
// 文字列、数値型、構造体など。他のenumを含むことさえできる
// Rustにはnullがない

// OptionのSomeがあれば、値が存在していることが分かり、Noneならばある意味nullを表している

// Optionの何が良いか
// Option<T>とT(ここでTはどんな型でもいい)は異なる型なので、
// コンパイラがOption<T>値を確実に有効な値かのようには使用させてくれません。
// 例えば、このコードはi8をOption<i8>に足そうとしているので、コンパイルできません。
// Option<i8>が確実に値を持っているかをコンパイラが保証できないから計算できない
/*
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
*/
// T型の処理を行う前には、Option<T>をTに変換する必要がある
