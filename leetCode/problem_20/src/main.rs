//working on it
struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        
        let byte = s.as_bytes();
        let mut stack = Vec::new();

        let left = [40, 123, 91];
        let right = [41 , 125, 93];

        if right.contains(&byte[0]) {
            return false;
        }
        
        for i in 0..(byte.len() + 1) {
            if left.contains(&byte[i]) {
                stack.push(byte[i]);
            }

            if right.contains(&byte[i]) {
                stack.last()
            }
        }
        true
    }
}

fn main() {
    println!("\n{}", Solution::is_valid(String::from("()")));
}
