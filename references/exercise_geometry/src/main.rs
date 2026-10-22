/*

calculate the magnitude of a vector by summing the squares of its coordinates
and taking the square root. use the sqrt() method to calculate the square root 
like v.sqrt()

*/

fn magnitude(v: &[f64; 3]) -> f64 {
    let mut mag = 0.0;

    for coord in v {
        mag += coord * coord;
    }

    mag.sqrt()
}

fn normalize(v: &mut [f64; 3]) {
    println!("Vector: {v:?}");
    let mut mag = magnitude(v);
    
    for coord in v {
        println!("coord: {coord:?}");
        // even tough coord is a references, rust println! macro 
        //automatically dereference it when printing
        println!("*coord: {}", *coord);
        *coord /= mag;
    }
}

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[3.0, 5.0, 6.0]));
    
    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
