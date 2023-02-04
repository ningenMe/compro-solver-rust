use proconio::input;
use std::cmp;

fn Yn(flg : bool) {
    println!("{}", if flg {"Yes"} else {"No"}); 
}

fn main() {
    input!{
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let mut blue: i64 = a;
    let mut red: i64 = 0;
    let mut ans: i64 = 0;
    while blue > red * d {
        blue += b;
        red += c;
        ans += 1;
        if ans >= 10000000 {
            println!("{}", -1);
            return;
        }
    }
    println!("{}", ans);
}
