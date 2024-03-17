fn main() {
    let number = 3;

    // ifは式
    // if式の条件式と紐付けられる一連のコードは、
    // 時として**アーム**と呼ばれることがある
    if number < 5 {  // 条件式は常にbool型でなければならない
        // pythonでいう 1がtrue, 0がfalse みたいな概念は罷り通らない
        println!("condition was true");       // 条件は真でした
    } else {  // elseはオプション
        println!("condition was false");      // 条件は偽でした
    }


    let number = 6;

    // 複数の条件分岐
    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is divisible by 2");
    } else {
        // 数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    };

    // ifは式だからこんなこともできる
    // ただし、各アームで評価される値は、全て同じ型でなければならない。
    // これにより、強力な型安全が保障される。 cargoちゃん優秀
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" };  // これはエラー

    // numberの値は、{}です
    println!("The value of number is: {}", number);
}
