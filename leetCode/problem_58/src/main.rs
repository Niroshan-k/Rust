struct Solution { }

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let bytes: Vec<u8> = s.as_bytes().to_vec();
        let mut count: i32 = 0;
        
        for i in (0..bytes.len()).rev() {
            if bytes[i] == 32 {
                continue;
            } else {
                for j in (0..(i+1)).rev() {
                    if bytes[j] == 32 {
                        break;
                    }
                    count += 1;
                }
                break;
            }
        }
        count  
    }
}

fn main() {
    let s = String::from("Hello World");
    let count = Solution::length_of_last_word(s);
    println!("count: {count}");
}

// problem with the code. AI help solution. above is my code 🫠

/*
The bottleneck is to_vec(). It allocates a brand new heap buffer for the 
bytes. Because String already owns a valid byte buffer, you can borrow it 
directly with s.as_bytes() for zero allocations ($O(1)$ space).

solution:
-------------------
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end()
            .bytes()
            .rev()
            .take_while(|&b| b != b' ')
            .count() as i32
    }
}
-------------------
1. No Heap Allocation: s.as_bytes() borrows the existing string slice 
without duplicating data.
2. Lazy Iteration: trim_end().bytes().rev() processes bytes lazily from the 
back without collecting or shifting indices.
3. LLVM Vectorization: High-level iterator combinators compile down to efficient 
pointer arithmetic matching or beating manual loops.

*/