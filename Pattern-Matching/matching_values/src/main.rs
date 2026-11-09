/*

The match keyword lets you match a value against one or more patterns. The patterns can be
simple values, similarly to switch in C and C++, but they can also be used to express more
complex conditions:

*/

#[rustfmt::skip]
fn main() {
    let input = 'x';
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0' ..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("Something else")
    }

    /*
        Match guards are different from if expressions after the =>. An if expression is
        evaluated after the match arm is selected. Failing the if condition inside of that block
        won't result in other arms of the original match expression being considered. In the
        following example, the wildcard pattern _ => is never even attempted.
    */
    let input = 'a';
    match input {
        key if key.is_uppercase() => println!("Uppercase"),
        key => if input == 'q' { println!("Quitting") },
        _ => println!("Bug: this is never printed"),
    }

    /*
        Note that you can't use an existing variable as the condition in a match arm, as it will
        instead be interpreted as a variable name pattern, which creates a new variable that
        will shadow the existing one. For example:
    */
    let expected = 5;
    match 123 {
        expected => println!("Expected value is 5, actual is {expected}"),
        _ => println!("Value was something else"),
    }

    /*
        Another piece of pattern syntax is the @ syntax which binds a
        part of a pattern to a variable. For example:
    */
    let opt = Some(123);
    match opt {
        outer @ Some(inner) => {
        println!("outer: {outer:?}, inner: {inner}");
        }
        None => {}
    }
}

/*

You might point out how some specific characters are being used when in a pattern

    – | as an or
    – .. matches any number of items
    – 1..=5 represents an inclusive range
    – _ is a wild card

*/