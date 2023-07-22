use proconio::input;

fn Yn(flg : bool) {
    println!("{}", if flg {"Yes"} else {"No"}); 
}

fn main() {
    input!{
        n: usize,
        mut A: [String; n]
    }
    let c = A.iter().filter(|&a| a == "and" || a=="not" || a=="that" || a=="the" || a=="you").count();
    let flg = c > 0;
    Yn(flg)
}
