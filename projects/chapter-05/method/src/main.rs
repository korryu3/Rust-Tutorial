// rectanglesの続き
// あのプログラムを、メソッドを使ってもっとリファクタリングする

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()  // Rectangleのメソッドを呼び出す
    );

    // 引数の多いメソッドを作ろう
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 関連関数を使って、構造体のインスタンスを作成
    let sq = Rectangle::square(3);
    println!(
        "The area of the rectangle is {} square pixels.",
        sq.area()  // Rectangleのメソッドを呼び出す
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation: 実装
impl Rectangle {
    // selfはメソッドが呼び出されている構造体インスタンスを表す
    // 引数が &Rectangle ではなく、 &self になっている！！
    // `&self`, `&mut self`, `self` 等の書き方ができる
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // impl内に、Rectangleの関連関数を定義
    // selfを持たないから、メソッドではなく関数扱い
    // 関連関数は、構造体の新規インスタンスを返すコンストラクタによく使用される
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// 各構造体には、複数のimplブロックを存在させることもできる
// 複数のimplブロックが有用になるケースはジェネリック型やトレイトを使うときに見るだろう
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }


// 自動参照および参照外しの部分がよくわからなかった

