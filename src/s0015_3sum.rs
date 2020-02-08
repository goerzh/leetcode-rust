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
                result.push(vec![nums[i], v.0, v.1]);
            }

            i += 1;
        }

        result
    }

    pub fn two_sum(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = Vec::new();
        let (mut i, mut j) = (0_usize, nums.len() - 1);
        while i < j {
            if nums[i] + nums[j] > target {
                j -= 1;
            } else if nums[i] + nums[j] < target {
                i += 1;
            } else {
                result.push((nums[i], nums[j]));
                i = Solution::next_unique(nums, i, true);
                j = Solution::next_unique(nums, j, false);
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