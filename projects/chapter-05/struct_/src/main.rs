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

    let email = String::from("example@example.com");
    let user2 = User {
        email,  // 普通の変数でもフィールド初期化記法は可能！
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user2.email(mut): {}", user2.email);
}




// 構造体は、各データ片に名前をつけるため、タプルよりも値の意味が明確
