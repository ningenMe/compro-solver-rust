use proconio::input;

fn Yn(flg : bool) {
    println!("{}", if flg {"Yes"} else {"No"}); 
}

fn f(t:i32) -> i32 {
    return t*t+2*t+3;
}
fn main() {
    input!{
        t:i32
    }
    println!("{}", f(f(f(t)+t)+f(f(t))) );
}
