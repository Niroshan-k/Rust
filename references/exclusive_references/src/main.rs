fn main() {

    let mut point = (1, 2);
    let x_coord = &mut point.0; // mutable reference to the first element of the tuple
    *x_coord = 20; // modify the value through the mutable reference

    //point.0 = 30; // getting compile error: cannot assign to `point.0` because it is borrowed

    println!("point: {x_coord:?}"); // prints: point: 20
}

// "Exclusive" means that only this reference can be used to access the value.
// no other references (shared or exclusive) can exist at the same time. This is enforced at compile time by the borrow checker.
// referenced value cannot be accessed while exclusive references exist.