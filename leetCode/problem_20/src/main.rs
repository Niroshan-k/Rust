//working on it
struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        
        let byte = s.as_bytes();
        let mut stack = Vec::new();
        let left = [40, 123, 91];
        let right = [41 , 125, 93];
        
        for i in 0..byte.len() {
            if left.contains(&byte[i]) {
                stack.push(byte[i]);
            } else if right.contains(&byte[i]) {
                let Some(last) = stack.pop() else {
                    return false;
                };
                let mut matched = false;
                for j in 0..3 {
                    if left[j] == last && byte[i] == right[j] {
                        matched = true;
                        break; // stop checking once matched
                    }
                }

                if !matched { return false; }
            }  
        }
        stack.is_empty()
    }
}

fn main() {
    println!("\n{}", Solution::is_valid(String::from("{}")));
}
