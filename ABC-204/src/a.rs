use proconio::input;

fn Yn(flg : bool) {
    println!("{}", if flg {"Yes"} else {"No"}); 
}

fn main() {
    input!{
        x: i32,
        y: i32
    }
    println!("{}", if x==y {x} else {0^1^2^x^y});
}
