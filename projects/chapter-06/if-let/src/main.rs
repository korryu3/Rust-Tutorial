fn main() {

    #[derive(Debug)] // すぐに州を点検できるように
    enum UsState {
        Alabama,
        Alaska,
        // ... などなど
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }


    // matchで一つの値にマッチした処理をしたい
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let構文
    // 上記のコードを簡潔に書きたい
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // ifとの違いは、列挙型の値を取り出せる点
    // ifは条件に基づいて実行, if letはパターンマッチで実行
    if let Some(x) = some_u8_value {
        println!("The value is {}", x);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        // {:?}州のクォーターコイン
        // coinはstateに所有権をmoveしてる
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // ↓ if letで書き換える
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}
