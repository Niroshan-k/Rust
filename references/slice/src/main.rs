fn main() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[1..4]; // a slice is a reference to a contiguous sequence of elements in a collection
    println!("s: {s:?}");
}

/* 
  -- slice borrow data from the sliced type.
  -- create a slice by borrowing a. and specifying a range of elements to include in the slice.
  -- &a[0..a.len()] and &a[..] are equivalent ways to create a slice that includes all elements of a.

  -- s is a references to a slice of i32s.
  -- s is no longer mention array length.
  -- allow to perform computation on slices of different sizes.

  -- slices always borrow data from another object, so in here a
  -- has to remain alive (in scope) for at least as long as our slice.
  -- you cant grow a slice once created.
    * cant append element of slice, or remove elements from it.
    * you cant grow a slice to point to a larger section of the backing buffer
    [slice does not have info about the length of the underlying buffer so you cant know how large the slice can grow.]
  
  -- to get a large slice you have to change where its start.

*/

/*
What is the "underlying buffer"?
 -- It is the actual memory storage holding the original data—in this example, the array a ([10, 20, 30, 40, 50, 60]).

What do these points mean overall?

    -- Slices are just windows: A slice (&[i32]) is a lightweight view into part of an array. It doesn't own the data; 
    it just points to where it starts and how long the view is.
    
    -- Flexible sizes: Because the type &[i32] doesn't include the array's length, 
    you can write functions that accept slices of any size.

    -- Why slices cannot grow: A slice only knows about its own specific window. 
    It has no record of the rest of the underlying buffer, so it cannot expand itself. 
    To see more data, you must create a new slice from the original buffer. 
*/