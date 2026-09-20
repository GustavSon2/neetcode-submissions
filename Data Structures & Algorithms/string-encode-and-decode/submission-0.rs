impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        
        let mut encoded_string = String::new();

    for s in strs {
        encoded_string.push_str(&format!("{}#{}", s.len(), s));
    }

    encoded_string
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut decoded_strs = Vec::new();
    let mut i = 0;

    while i < s.len() {
        // Find the '#'
        let delimiter = s[i..].find('#').unwrap() + i;

        // Parse the length
        let len: usize = s[i..delimiter].parse().unwrap();

        // Move past '#'
        let start = delimiter + 1;
        let end = start + len;

        // Extract the original string
        decoded_strs.push(s[start..end].to_string());

        i = end;
    }

    decoded_strs
    }
}
