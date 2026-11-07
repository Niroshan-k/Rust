// type aliases creates a name for another type.
// the two types can be used interchangeably.

enum CarryableConcreateItem {
    Left,
    Right
}

type Item = CarryableConcreateItem;
/*
 Every single place you write Item, the compiler mentally swaps it back to CarryableConcreteItem. 
 They are 100% interchangeable. If a function expects a CarryableConcreteItem, you can pass your 
 Item alias right into it without the compiler batting an eye.
 */

// Aliases are more useful with long, complex types:
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;

/*
Without an alias, every time you want to pass a player's inventory into a function signature, 
you would have to type out that entire 25-character nested type. 
An alias acts like a clean label for a complex data structure.

Let's quickly dissect what that nested type actually means under the hood:
    - Item: Your base enum (e.g., Left, Right).
    - RefCell<Item>: Gives you interior mutability, meaning you can mutate the item even 
    if you only have an immutable reference to it (checked at runtime).
    - Arc<...>: An Atomically Reference Counted pointer, meaning multiple threads can 
    safely share ownership of this item across CPU cores.
    - Vec<...>: A growable array living on the heap holding all those shared items.
    - RwLock<...>: A thread-safe Read-Write lock that ensures multiple threads can read the 
    inventory at once, but only one thread can write to it at a time.

Typing that out everywhere would be a nightmare. PlayerInventory saves your sanity.
 */

fn main() {
    println!("Hello, world!");
}

/*

The Danger of Type Aliases (Why Newtypes are Better)

The slide mentions [64page]:
    "A newtype is often a better alternative since it creates a distinct type. 
    Prefer struct InventoryCount(usize) to type InventoryCount = usize."

Because a type alias is just a nickname, it offers zero type safety.
Imagine you write this code using a type alias:

Rust code ->

    type InventoryCount = usize;
    type PlayerHealth = usize;

    fn give_damage(health: PlayerHealth) { /* ... */ }
    
    let count: InventoryCount = 5;
    give_damage(count); // Oops! 

Because InventoryCount and PlayerHealth are both just aliases for usize, 
Rust allows you to accidentally pass an inventory count into a health function. 
The compiler thinks: "A usize is a usize, looks fine to me!" This is how bugs happen.

The Solution: The "Newtype" Pattern

Instead of a type alias, you wrap the primitive in a tiny tuple struct:

Rust code ->

    struct InventoryCount(usize);
    struct PlayerHealth(usize);

Now, InventoryCount and PlayerHealth are completely distinct types. 
If you try to pass an InventoryCount into a function expecting a PlayerHealth, the compiler 
will violently reject it:
    “Mismatched types: expected struct PlayerHealth, found struct InventoryCount!”

The best part? At runtime, a newtype tuple struct like InventoryCount(usize) has zero performance 
overhead. The compiler strips away the struct wrapper during compilation and treats it as a raw usize, 
but gives you strict, compile-time safety guards while you write your code.

 -> Use Type Aliases (type X = Y) when you want a neat shortcut for a long, repetitive, or 
 complex type signature.
 -> Use Newtypes (struct X(Y)) when you want absolute safety so you don't accidentally mix 
 up things like IDs, counts, and measurements!
*/
