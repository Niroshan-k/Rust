fn main() {
    // 1. Create a raw pointer pointing to memory address 0 (null)
    let null_ptr: *mut u8 = std::ptr::null_mut();
    
    // 2. Printing the address works fine! 
    // {:p} formats it as a pointer address. This will print 0x0.
    println!("The null pointer address is: {:p}", null_ptr);

    // 3. Entering the danger zone: unsafe block
    unsafe {
        // DANGER: Trying to dereference (read the value at) address 0x0.
        // This is Undefined Behavior and will instantly crash your program with a Segmentation Fault!
        let val = *null_ptr; 
        println!("The value is: {}", val);
    }
}