
/*
    -- &str is a slice of UTF-8 encode bytes, similar to &[u8].
    -- String is an owned buffer of UTF-8 encode bytes, similar to Vec<u8>.
    -- &str is a reference to a string slice, which is a view into a string
    -- String is a growable, heap-allocated data structure used to store string data.
*/

fn main() {
    let s1: &str = "World"; // string slice, a reference to a string literal
    println!("s1: {s1:?}");

    let mut s2: String = String::from("Hello"); // owned string, a heap-allocated string
    println!("s2: {s2:?}");

    s2.push_str(s1); // append string slice to owned string
    println!("s2: {s2:?}");

    let s3: &str = &s2[2..9]; // string slice, a reference to a portion of the owned string
    println!("s3: {s3:?}");
}

/*
    -- &str introduces a string slice, immutable reference to UTF-8 encoded string data stored in a 
    block of memory. String literals ("Hello"), are stored in the program's binary.
    -- String type is a wrapper around a vector of bytes. as with a Vec<T>, it is owned.
    -- String::from() create a string from string literal; 
    -- String::new() creates an empty string, we can use push() or push_str() to add data to it.

    -- format!() macro can be used to create a String from a string literal and other values.

    -- when borrow &str from String via &, if not aligned with character boundaries, the expression will panic at runtime.
    the chars iterator iterates over the characters of a string slice, and preferred over trying to get character boundaries right.

    -- String is a rough equivalent of std::string from c++ [difference - it can only contain UTF-8 encoded bytes and never use a 
    small-string optimization ].

    --------------------------------------

    &str (String Slice):
    An immutable view into UTF-8 text. It doesn't own the data; it just points to it. 
    String literals like "World" are hardcoded directly into the program's binary executable.

    String (Owned String):
    A growable, heap-allocated buffer of UTF-8 bytes (s2). Because it is owned, 
    you can modify it (e.g., using .push_str(), .push(), or the format!() macro).

    Borrowing a slice from a String (&s2[2..9]):
    --You can create a &str view of a String using a range.
    [Warning: Rust indices are based on bytes, not characters. If you slice right through the middle of a multi-byte UTF-8 character
    (like an emoji or accented letter), your code will panic at runtime. Use the .chars() iterator to handle characters safely instead. ]

    C++ Comparison:
    -- &str is like C++ std::string_view (guaranteed to point to valid UTF-8 text).
    --String is like C++ std::string (an owned, heap-allocated string, though Rust's version 
    strictly mandates UTF-8 and doesn't use small-string optimization).

    you cannot edit an &str for two main reasons:
    -- It's immutable: Standard references (&) in Rust are immutable by default, 
    meaning you cannot change the data they point to through that reference.
    --It doesn't own the memory: String literals (like "World") are hardcoded directly into the program's binary file, 
    which is read-only memory. Furthermore, because a slice doesn't own its backing buffer, 
    it cannot resize or rearrange characters.

    References Validity:
    1. references can never be null, dangling, or uninitialized.
    2. references can't outlive the data they point to.
    3. references must always point to valid, initialized data.
*/
