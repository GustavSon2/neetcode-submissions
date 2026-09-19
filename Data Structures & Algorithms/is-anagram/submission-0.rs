use std::collections::HashSet;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        
        let mut found = false;

        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();

        for i in 0..s.len() {
            vec1.push(s.chars().nth(i).unwrap());
            
        }

        for j in 0..t.len() {
            
            vec2.push(t.chars().nth(j).unwrap());
        }
        vec1.sort();
        vec2.sort();

        
        if vec1 == vec2 {
            found = true;
        }
        found
    }
}
