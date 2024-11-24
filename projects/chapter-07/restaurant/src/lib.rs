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
            // ../../eat_at_restaurant みたいな感じ
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
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // enumを公開すると、enumのすべてのバリアントが公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


// useを使ってmoduleをスコープに持ち込む
// use crate::front_of_house::hosting;
// 相対パスで指定可能
use self::front_of_house::hosting;  // 自ファイル内のモジュールを指定する場合はselfを使える
// 関数を指定することも可能だが、module内で定義されているのかが分かりにくいのであまり使わない方が良い
use self::front_of_house::hosting::add_to_waitlist;

// ただ、構造体, enum, その他要素を指定する場合はフルパスで書くのが慣習としてある
use std::collections::HashMap;

pub fn eat_at_restaurant() {
    // 絶対パス
    // crateがルートパス
    // ファイルシステムでいうと /front_of_house/hosting/add_to_waitlist みたいな感じ
    crate::front_of_house::hosting::add_to_waitlist();
    // 相対パス
    front_of_house::hosting::add_to_waitlist();

    // useでスコープに持ち込んだので、関数名だけで呼び出せる
    hosting::add_to_waitlist();


    // 夏 (summer) にライ麦 (Rye) パン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // やっぱり別のパンにする
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 下の行のコメントを外すとコンパイルできない。食事についてくる
    // 季節のフルーツを知ることも修正することも許されていないので
    // meal.seasonal_fruit = String::from("blueberries");

    let oder1 = back_of_house::Appetizer::Soup;
    let oder2 = back_of_house::Appetizer::Salad;
}

// 同じ名前の要素があるとエラーになるため、下記のようにエイリアスをつけることができる
use std::fmt::Result;
use std::io::Result as IoResult;

// useをネストさせることができる
// use std::cmp::Ordering;
// use std::fmt;
// ↓
use std::{cmp::Ordering, fmt};

// use std::io;
// use std::io::Write;
// ↓
use std::io::{self, Write};

// glob演算子
// use std::collections::*;  // testされるあらゆるものを tests モジュールに持ち込む時とかに便利

// pub useについて
/*
// lib.rs
mod internal {
    pub struct MyStruct;
}

// モジュールをそのまま公開しない場合
pub use internal::MyStruct;

// main.rs
// pub useがない場合
use my_library::internal::MyStruct; // 内部構造を知る必要がある

// pub useがある場合
use my_library::MyStruct; // シンプルにアクセス可能
*/
