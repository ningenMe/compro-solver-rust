use proconio::input;
use std::cmp;

fn Yn(flg : bool) {
    println!("{}", if flg {"Yes"} else {"No"}); 
}

fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32,
    }
    println!("{}", a+b+c-cmp::min(cmp::min(a,b),c));
}
