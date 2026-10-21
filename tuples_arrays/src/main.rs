// try to get a error that runtime checks failing
// will compile but panic at runtime
fn get_index() -> usize {
    3
}

fn check_order(tuple: (i32, i32, i32)) -> bool {
    let (left, middle, right ) = tuple;
    left < middle && middle < right
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let len = matrix.len();
    let mut transpose = [[0; 3]; 3];
    for i in 0..len {
        for j in 0..len {
            transpose[j][i] = matrix[i][j];
        }
    }
    transpose
}

fn main() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[get_index()] = 0;

    //tuple
    let t: (i8, bool, String) = (7, true, String::from("hii"));

    // {} this gives the default output {:?} gives the debug output
    // int, str implement the default output
    // but arrays only implement debug output
    println!("a: {a:?}");

    dbg!(t.0);
    dbg!(t.1);
    dbg!(t.2);

    //array iteration
    let primes = [2,3,5,7,11,13,17,19];
    // 2..prime -> if prime is 7 -> 2..7 -> 2,3,4,5,6
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }

    let tuple = (1, 5, 7);
    println!("{tuple:?}: {}", if check_order(tuple) { "ordered" } else { "unordered" });
    
    // nested array ex:
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("Original:");
    for row in matrix { println!("{row:?}"); }
    
    let transposed = transpose(matrix);
    
    println!("\ntranspose:");
    for row in transposed { println!("{row:?}"); }
}
