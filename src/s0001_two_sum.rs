use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut pair: HashMap<i32, i32> = HashMap::new();

        for (i, elm) in nums.iter().enumerate() {
            if pair.contains_key(elm) {
                result.push(pair.get(elm).unwrap().clone());
                result.push(i as i32);
            } else {
                pair.insert(target - elm.clone(), i as i32);
            }
        }

        result
    }
}