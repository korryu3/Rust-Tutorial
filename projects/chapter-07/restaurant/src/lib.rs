// front_of_houseは公開されていませんが、
// eat_at_restaurant関数はfront_of_houseと同じモジュール内で定義されている
// （つまり、eat_at_restaurantとfront_of_houseは兄弟な）ので、
// eat_at_restaurantからfront_of_houseを参照することができます。
// モジュールツリーを考えると良く分かる
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            seat_at_table();
            crate::front_of_house::hosting::seat_at_table();
        }

        fn seat_at_table() {
            // superはこういうこともできる
            super::super::eat_at_restaurant();
        }
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


fn serve_order() {}

mod back_of_house {

    fn fix_incorrect_order() {
        crate::serve_order();

        // 兄弟なので、パス単体でアクセス可能
        cook_order();
        // 親モジュール(この場合crate)から見たパスでアクセス可能
        super::serve_order();
    }

    fn cook_order() {}


    // モジュールの構造体
    pub struct Breakfast {
        // フィールドごとに公開の可否を設定できる
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}



pub fn eat_at_restaurant() {
    // 絶対パス
    // crateがルートパス
    // ファイルシステムでいうと /front_of_house/hosting/add_to_waitlist みたいな感じ
    crate::front_of_house::hosting::add_to_waitlist();
    // 相対パス
    front_of_house::hosting::add_to_waitlist();


    // 夏 (summer) にライ麦 (Rye) パン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // やっぱり別のパンにする
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    println!("readonly field -> {}", meal.seasonal_fruit);

    // 下の行のコメントを外すとコンパイルできない。食事についてくる
    // 季節のフルーツを知ることも修正することも許されていないので
    // meal.seasonal_fruit = String::from("blueberries");
}
