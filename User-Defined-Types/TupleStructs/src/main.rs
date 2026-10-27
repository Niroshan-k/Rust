struct Point(i32, i32);

// this is often for single-field wrappers - newtypes
struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("i dont knoww duhhh")
}

fn set_thruster_force(force: Newtons) {
    // ...
}

fn main() {
    // if the field names are unimportant, can use tuple struct
    let p = Point(17, 23);
    println!("({p.0}, {p.1})");

    let force = compute_thruster_force();
    set_thruster_force(force);
}

/*

Newtypes are a great way to encode additional information about the value in 
a private type.

A Newtype is simply a custom struct that wraps a single primitive value 
(like an f64 or String) to give it a specific meaning.
Why use them?

    Type Safety (Preventing disasters): Under the hood, 
    both PoundsOfForce and Newtons are just a plain f64 number. 
    But Rust treats them as completely separate types. 
    You can accidentally pass pounds into a function expecting Newtons.
    (This is a nod to the Mars Climate Orbiter crash, 
    where a mix-up between pounds and newtons caused a $125M 
    spacecraft to burn up!)

    No Implicit Mix-ups: The compiler forces you to be explicit. 
    If a function wants Newtons, you can't just hand it PoundsOfForce 
    without converting it first.
*/
