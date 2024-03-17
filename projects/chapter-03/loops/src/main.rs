// これは無限ループ
// fn main() {
//     loop {
//         println!("again!");   // また
//     }
// }


// ループ内にループがある場合、breakとcontinueは最も内側のループに適用されます。
// ループラベルを使用することで、breakやcontinueが適用されるループを指定することができます。
fn main() {
    let mut count = 0;
    // 'label: loop {}  // シングルクォーテーションじゃなきゃダメ
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;  // この内側のループを抜ける
            }
            if count == 2 {
                break 'counting_up;  // ラベルがついたループから抜ける
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);


    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    // これは、以下のloopと同義
    /*
    loop {
        if number == 0 {
            break;
        }
        println!("{}!", number);

        number -= 1;
    }
    */

    // 発射！
    println!("LIFTOFF!!!");


    let  a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // 標準ライブラリのRange型
    // 片方の数字から始まって、 もう片方の数字未満の数値を順番に生成する型
    for number in (1..4).rev() {  // revはRange型のメソッド。逆順にする
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}


// loop: 条件がない無限ループ. breakでぶっ壊すしかない
// while: 条件がある無限ループ. breakでも抜けれる
// for: 配列とかをぶん回すループ. 多分breakでも抜けれる
