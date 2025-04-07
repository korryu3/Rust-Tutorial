fn main() {
    // データがないときは型注釈が必須
    // Vecは<T>で定義されるため、任意の型が指定可能
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // データから型を推測するため、型注釈は不要
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("{:?}", v);
    {
        let v = vec![1, 2, 3];
    }  // ベクタと要素がドロップされる

    // ベクタの要素にアクセス
    let v = vec![1, 2, 3, 4, 5];

    // 所有権が移らないように参照を使う
    // i32だとCopyトレイトが実装されているため、参照を使わなくても所有権は移らない
    // Stringとかだと、所有権が渡されて、パニックになる可能性がある
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // get()はOption<&T>が返ってくる
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let v = vec![String::from("hello")];
    // let x = v[0]; // エラー: 所有権が移るのでVecが壊れる

    // let does_not_exist = v[100];  // エラー
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist); // None

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6);  // エラー: 不変参照と可変参照が同時に存在するので、借用チェッカーが働く
    // 要素の追加時にメモリのスペースが無ければ、メモリの再確保が行われて、参照が無効になるから
    println!("The first element is {}", first); // エラー: 参照が無効になる

    // 不変参照でforループ
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // 可変参照でforループ
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // *: 参照外し演算子
        *i += 50;
        println!("{}", i);
    }

    // enumを使えば、vecで複数の型を保持できる
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // 事前にSpreadsheetCellの定義が分かってる際に有効
    // ベクタに格納し得るすべての型を網羅できない場合は、enumではなくトレイトを使ったアプローチがある
}
