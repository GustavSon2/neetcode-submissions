impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut vec1 = Vec::new();

        for i in 0..nums.len()-1 {
            
            for j in i..nums.len() {
                if nums[i] + nums[j] == target && i != j {
                    vec1.push(i as i32);
                    vec1.push(j as i32);
                    break;
                }
            }
        }
        vec1
    }
}
