impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut found = false;
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] == nums[j] {
                    found = true;
                }
            }
        }
    found
    }
}
