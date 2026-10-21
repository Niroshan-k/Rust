fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the string, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to an array of bytes

    for ( i, &item) in bytes.iter().enumerate() { // enumerate wraps the result of iter and return
                                                  // element as part of a tupple, index and
                                                  // reference to the element
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


