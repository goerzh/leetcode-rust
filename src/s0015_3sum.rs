use std::collections::{HashMap, HashSet, BTreeSet};

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        nums.sort();
        let mut previous = nums[0] - 1;
        let mut i = 0;
        while i < nums.len() - 2 {
            if nums[i] == previous {
                i += 1;
                continue;
            }

            previous = nums[i];

            let mut temp = Solution::two_sum(&nums[i+1..], 0-nums[i]);
            for v in temp.iter_mut() {
                v.insert(0, nums[i]);
            }
            result.extend(temp);

            i += 1;
        }

        result
    }

    pub fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut pair: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut i = 0;
        while i < nums.len() {
            if pair.contains_key(&nums[i]) {
                let mut temp: Vec<i32> = Vec::new();
                temp.push(pair.get(&nums[i]).unwrap().clone());
                temp.push(nums[i]);

                result.push(temp);

                i = Solution::next_unique(nums, i, true);
            } else {
                pair.insert(target - nums[i], nums[i].clone());

                i += 1;
            }
        }

        result
    }

    pub fn next_unique(nums: &[i32], idx: usize, forward: bool) -> usize {
        let curr = nums[idx];
        let mut i = idx;
        while i > 0 && i < nums.len() && nums[i] == curr {
            i = if forward {i + 1} else { i - 1 }
        }
        i
    }
}


mod tests {
    #[test]
    pub fn test_solution() {
        use super::*;
//        assert_eq!(Solution::three_sum(vec![0,-4,-1,-4,-2,-3,2]),
//                   vec![vec![-2, 0, 2]]);
//        assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
//                   vec![vec![-1, 0, 1], vec![-1, -1, 2]]);
        assert_eq!(Solution::three_sum(vec![0,0,0,0]),
                   vec![vec![0,0,0]]);
    }
}