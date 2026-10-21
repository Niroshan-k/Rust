// a reference provides a way to access another value 
// without taking ownership of the value, and is also 
// called "borrowing". shared references are read-only, reference
// data cannot change.


fn main() {
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;
    dbg!(r);

    r = &b;
    dbg!(r);

    *r = 'X';
}

// a shared reference to a type T has type &T.
// a ref value is made with the & operator.
// the * operator "dereferences" a reference, yielding its value.
