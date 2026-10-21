fn ev(mut num: u32) -> u32 {
    num = num / 2;
    num
}

fn odd(mut num: u32) -> u32 {
    num = ( 3 * num ) + 1;
    num
}

fn coll(mut n: u32) -> u32 {
    
    let mut seq = vec![n];

    while n != 1 
    {
        if n % 2 == 0 {
            n = ev(n);
            seq.push(n);
            coll(n);
        } else {
            n = odd(n);
            seq.push(n);
            coll(n);
        }
    }
    seq.len() as u32
}

//another solution
fn collatz_len(mun n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        // rust if is an expression, it produces a value.
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

fn main() {
    let n = 11;
    println!("{}", coll(n));
}
