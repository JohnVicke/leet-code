use std::collections::HashMap;

pub fn solve(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_to_index = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let remaining = target - num;
        if let Some(&prev_index) = num_to_index.get(&remaining) {
            return vec![prev_index as i32, index as i32];
        }
        num_to_index.insert(num, index);
    }
    vec![]
}
