impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
/*
        let mut vector = Vec::new();
        let length = nums.len();
        
    
        for i in 0..length {
            let mut res = 1;
            let mut j = i+1;
        while j % length != i % length {
            res *= nums[j % length];
            j += 1;
        }
        vector.push(res);
        }
        vector
        
    }*/
    let length = nums.len();
    let mut vector = vec![1; length];

    let mut prefix = 1;

    for i in 0..length {
        vector[i] *= prefix;
        prefix *= nums[i];
    }

    let mut suffix = 1;

    for i in (0..length).rev() {
        vector[i] *= suffix;
        suffix *= nums[i];
    }
    
    vector
    }

   
}


