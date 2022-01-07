use proconio::input;

fn Yn(flg : bool) {
    println!("{}", if flg {"Yes"} else {"No"}); 
}

fn main() {
    input! {
        mut s: i32,
        mut t: i32,
        mut x: i32
    }
    let mut flg = false;
    for i in s..s+24 {
        if i%24 == t {
            break
        }
        if i%24 == x {
            flg = true
        }
    }
    Yn(flg);
}
