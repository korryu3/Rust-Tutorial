fn main() {
    println!("Hello, world!!");
    cvt_cel_or_fah("C", 9.);  // 48.2 °F
    cvt_cel_or_fah("F", 59.);  // 15 °C

    println!("フィボナッチ数列{}番目: {}", 5, fibo_n(5));  // 3
    println!("フィボナッチ数列{}番目: {}", 10, fibo_n(10));  // 34
}


// 温度を華氏と摂氏で変換する。
fn cvt_cel_or_fah(disc: &str, temp: f32) {
    if disc == "C" {
        let cel = temp;
        let fah = cel * 1.8 + 32.;
        println!("摂氏から華氏への変換: {}°C -> {}°F", cel, fah);
    }
    if disc == "F" {
        let fah = temp;
        let cel = ( fah - 32. ) / 1.8;
        println!("華氏から摂氏への変換: {}°F -> {}°C", fah, cel);
    }
}


// フィボナッチ数列のn番目を生成する。
fn fibo_n(n: usize) -> usize{
    let n = n - 1;
    let mut fibo_ls: Vec<i32> = vec![0, 1];
    for i in 2..=n {
        let fibo_n_val = fibo_ls[i-2] + fibo_ls[i-1];
        fibo_ls.push(fibo_n_val);
    }
    fibo_ls[n].try_into().unwrap()
}


