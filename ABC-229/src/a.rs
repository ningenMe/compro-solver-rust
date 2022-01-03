use proconio::input;

fn Yn(flg : bool) {
    println!("{}", if flg {"Yes"} else {"No"}); 
}

fn main() {
    input! {
        s1: String,
        s2: String,
    }
    let mut flg = true;
    if s1 == "#." && s2 == ".#" {
        flg = false;
    }
    if s2 == "#." && s1 == ".#" {
        flg = false;
    }
    Yn(flg);
}
