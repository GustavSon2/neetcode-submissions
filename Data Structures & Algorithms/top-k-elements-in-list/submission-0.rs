use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
       let mut dict = HashMap::new();

    for num in nums {
        *dict.entry(num).or_insert(0) += 1;
    }

    let mut items: Vec<(i32, i32)> = dict.into_iter().collect();

    items.sort_by(|a, b| b.1.cmp(&a.1));

    items
        .into_iter()
        .take(k as usize)
        .map(|(num, _count)| num)
        .collect()
    }
}
