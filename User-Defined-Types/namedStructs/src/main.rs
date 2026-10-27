// rust has support for custom structs like c/c++

struct Person {
    name: String,
    age: u8
}

fn describe(person: &Person) {
    println!("{} is {} years old.", person.name, person.age);
}

fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);

    let jackie = Person {
        name: String::from("jackie"), ..avery 
    };
    // it allow us to copy the majority of the fields from the old
    // struct without having to type it all. 
    // it must always be the last element
}

/*

there is different type of structs.

1. Zero-sized structs (e.g. struct Foo;)
-- might be used when implementing a trait on some type 
but don't have any data that you want to store in the value itself.

-------

if you already have variable with the right names,
then you can create the struct using a shorthand.

struct fields do not support default values. default values are specified by
implementing the Default trait.
*/
