
fn takes_tuple(tuple: (char, i32, bool)) {
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;

    // do the same this as above
    //let (a, b, c) = tuple;

    // ignore the 1st element, only bind other
    //let (_, b, c) = tuple;

    //let(.., c) = tuple; // ignore all but the last element
    
    println!("{tuple:?}, {a}, {b}, {c}");
}

//array woek as well
fn takes_array(array: [u8; 5]) {
    let [first, .., last] = array;

    println!("{array:?}, {first}, {last}");
}

fn main() {
    println!("Hello, world!");
    takes_tuple(('a', 777, true));
    takes_array([5, 5, 6, 8, 3]);
}

/*

• All of the demonstrated patterns are irrefutable, meaning that they will always match
      the value on the right hand side.
    • Patterns are type-specific, including irrefutable patterns. Try adding or removing an
      element to the tuple and look at the resulting compiler errors.
    • Variable names are patterns that always match and bind the matched value into a new
      variable with that name.
    • _ is a pattern that always matches any value, discarding the matched value.
    • .. allows you to ignore multiple values at once.

*/