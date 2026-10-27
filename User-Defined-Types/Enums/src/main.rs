// enum keyword allows the creation of a 
// type which has a few different variants:

/*
 direction is a type with variants.
 - two values of Sirections -> left and right.
*/
#[derive(Debug)]
enum Direction {
    Left, Right,
}

/*
 type with a 3 variants. in addition to the payloads,
 rust will store a discriminant so that it knows at runtime which variant
 is in a playermove value.
*/
#[derive(Debug)] 
enum PlayerMove {
    Pass, // simple variant
    Run(Direction), // tuple variant
    Teleport { // struct variant
        x: u32, y: u32
    },
}

/*
    controlling discriminents:

    by default, rust choose the integer size for you. but if you are interacting
    with C code or need specific numbers like B=10000, you can force Rust's hand
    using #[repr(u32)].

    - because 10001 is too big to fit in a 1-byte or 2-byte integer, rust
    automatically scales the discriminant size up so it can hold the large
    numbers you specified.
*/
#[repr(u32)]
enum Bar {
    A, // 0
    B = 10000,
    C, // 10001
}

/*

Why A is 0: By default, Rust automatically numbers enum variants starting 
at 0 (A = 0, and normally B = 1).Why C is 10001: When you manually override 
a variant with a big number (B = 10000), Rust adapts the size of the enum 
to fit it, and it continues counting upwards from whatever number you gave it. 
Because B is 10000, the very next item (C) automatically 
becomes 10000 + 1 = 10001.

 */

fn main() {
    let dir = Direction::Left;
    let player_move: PlayerMove = PlayerMove::Run(dir);
    println!("On this turn: {player_move:?}");

    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);
}

/*
 - enums [enumerations] allow us to collect a set of values under one type.

 Struct & Enums:
 - in both, you can have a simple version without fields (unit struct) or
   one with different types of fields (variant payloads)
 - rust uses minimal space to store the discriminant.
  1. if necessary, it stores an integer of the smallest required size
 
1. what is a "Discriminant"?
an enum can hold different things at different times. to track of which 
varient is currently active, rust adds a hidden tag behind the scenes called
discriminant (tiny interger 0, 1 or 2).

2.minimal spaces & smallest size
rust doesn't waste memory. if an enum only has 3 variants, rust doesn't
use a massive 4 byte integer for the discriminant; its auto use smallest 
possible size (1 byte [u8]). 1 byte can hold -> 256 different values (2^8)

3. the "Niche Optimization" 
-the briliant memeory optimization in rust
* think a reference like &u8. a valid references can never be null pointer (0x0).
* because it is an impossible/invalid bit pattern for a reference, rust 
uses that exact unused "niche" to represent None in an Option<&u8>. An Option<&u8>
takes up the exact same amount of memory as a single raw pointer [8bytes]. there
is zero overhead for the Option wrapper.
*/


// but i had some questions:

/*
    * how can rust use a null references if safe rust doesn't allow them?
    1. in safe rust:
    you are strictly forbidden from creating or using a null reference.
    2. at the machine level [under the hoos]:
    computers don't know what "rust" is; they just see raw memory address 
    (numbers like 0x0, 0x7FFF, etc). address 0x0 (zero) is the classic 
    "null pointer" in Computer Science.
    3. compiler trick:
    the rust compiler looks at a type like Option<&u8>. it knows that a 
    valid reference can never be 0x0. so the compiler says: "hey, the bit
    pattern 0x0 is completely unused for valid references. i will steal 0x0
    to mean None. any other address means Some" .

    * what does "Zero overhead" mean?
    - means you don't pay a performance or memory 
    penalty for writing cleaner code
    In Rust, because of the niche optimization, an Option<&u8> takes up 
    the exact same 8 bytes in memory as a raw pointer. 
    There is zero extra memory wasted for the wrapper.
*/

/*
 * In many other languages, an optional reference (Option<&u8>) has to 
 carry an extra hidden flag behind the scenes to answer the question: 
 "Is this null or not?". That extra flag usually takes up a whole 
 extra 8 bytes of memory due to hardware alignment.
 
 Because Rust knows that address 0x0 (the null pointer) is a 
 forbidden dead zone that no valid reference can ever use, it stuffs 
 the "is it None or Some?" status directly into the address itself by 
 using 0x0.
 
     - Without this trick: 16 bytes (8 bytes for the address + 8 bytes 
     for the extra tag).
     - With Rust's niche optimization: Exactly 8 bytes.
 */