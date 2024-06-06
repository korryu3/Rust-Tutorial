fn main() {

    struct User {  // struct構文は式
        username: String,  // データ片の名前と型を定義する。これを"フィールド"と呼ぶ
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 構造体をインスタンス化する
    // 構造体から特定の値を得るには、ドット記法
    let user1 = User {
        email: String::from("someone@example.com"),  // フィールドの順序は問わない
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };  // インスタンス自体は式であるが、代入は文であるから、ここにセミコロンがある。
    println!("user1.email: {}", user1.email);

    // 一部のフィールドのみを可変にするのはできない！
    let mut user1 = User {  // インスタンス化するときは、可変・不変両方できる
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("user1.email(mut): {}", user1.email);

    // User構造体はmainの中にあるから、定義する場所はmainの中
    fn build_user(email: String, username: String) -> User {
        // フィールド名と引数(変数)が同じ名前で毎回書くのは煩わしい！
        // もっとフィールドが増えて複雑化したときも大変！
        // そんな時にフィールド初期化記法！
        User {
            email,  // email: email と同義。煩わしい記法とはもうおさらば！
            username,  // username: username と同義
            active: true,
            sign_in_count: 1,
        }
    }

    // インスタンスの値を更新して、新しいインスタンスを作りたい
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count
    };
    // 上と下は同じ事をしている
    // 構造体更新記法
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1  // 明示的にセットしていないフィールドが、与えられたインスタンスのフィールドと同じ値になる
    };

    // フィールドの型だけのタプル構造体
    // フィールドが同じ型でも、これ自体は型が違う扱いになる。
    // 引数にColorを受け取る関数は、Pointを実引数に受け取れない
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.1);

    // フィールドの型には、ライフタイムをしていしないと参照は使えない. *1
    // struct ErrorUser {
    //     email: &str,
    //     username: &str
    // }


}




// 構造体は、各データ片に名前をつけるため、タプルよりも値の意味が明確

// 何もかも空の構造体(ユニット様構造体)を定義できる

/*
*1
    構造体のインスタンスには、全データを所有してもらう必要がある
    このデータは、構造体全体が有効な間はずっと有効である必要があるのです。
    参照を持たせることもできるが、それをするにはライフタイムを使用しなければならない
    ライフタイムのおかげで構造体に参照されたデータが、構造体自体が有効な間、ずっと有効であることを保証してくれるのです。
*/
