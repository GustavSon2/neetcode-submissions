use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        for word in strs {
            let mut key: Vec<char> = word.chars().collect();
            key.sort();

            groups.entry(key).or_default().push(word);
        }

        groups.into_values().collect()
    }
}
