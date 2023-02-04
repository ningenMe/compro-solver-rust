use proconio::input;

fn Yn(flg : bool) {
    println!("{}", if flg {"Yes"} else {"No"}); 
}

fn main() {
    input!{
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        mut A: [i32; n]
    }
    let mut B = vec![0; n];
    for i in 0_usize..n {
        B[i] = A[i];
        if p-1<= i && i < q {
            B[i] = A[r-1 + i-(p-1)]
        }
        if r-1 <= i && i < s {
            B[i] = A[p-1 + i-(r-1)]
        }
    }
    for &e in &B {
        print!("{} ", e);
    }
    println!();
}
