struct Move {
    delta: (i32, i32),
    repeat: u32
}

#[rustfmt::skip]
fn main() {
    let m = Move { delta: (10, 0), repeat: 5 };
    match m {
        Move { delta: (0,0), .. } => println!("Standing still"),
        Move { delta: (x,0), repeat } => println!(" {repeat} step x: {x}"),
        Move { delta: (0,y), repeat: 1} => println!("Single step y: {y}"),
        _ => println!("Other move")
    }
}
