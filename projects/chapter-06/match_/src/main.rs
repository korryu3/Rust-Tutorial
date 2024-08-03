// match式でenumを有効活用しよう！

fn main() {
    // コインの種類
    enum Coin1 {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents1(coin: Coin1) -> i32 {
        match coin {
            // この一つ一つをアームと呼ぶ.
            // 構文: パターン => コード
            Coin1::Penny => {  // 複数コードを実行したい場合はブロックで囲う
                println!("Lucky penny!");
                1
            },
            Coin1::Nickel => 5,
            Coin1::Dime => 10,
            Coin1::Quarter => 25,
        }
    }

    // enumを値を紐づける場合

    #[derive(Debug)] // すぐに州を点検できるように
    enum UsState {
        Alabama,
        Alaska,
        // ... などなど
    }

    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents2(coin: Coin2) -> u32 {
        match coin {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {  // この列挙子に紐づいた値を state 変数に束縛させる
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    value_in_cents2(Coin2::Quarter(UsState::Alaska));

    // Option<T>のマッチ
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),  // Some列挙子で入ってきた値は、変数iに結びつく
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

}

// match式は常に包括的
// 例えば、以下のようなプログラムはパニックを起こす
// 全てのパターンに対して処理を書かなければならない
// その代わり、どんな値にもマッチする「_」パターンがある
/*
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // Noneを処理するアームがない！！
        Some(i) => Some(i + 1),
    }
}
*/
