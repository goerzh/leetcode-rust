use std::i32;

struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut nums = nums;
        while i < i32::max_value() {
            if Solution::three_sum(&mut nums, target + i) {
                return target + i;
            } else if Solution::three_sum(&mut nums, target - i) {
                return target - i;
            }
            i += 1;
        }
        i
    }

    pub fn three_sum(nums: &mut Vec<i32>, target: i32) -> bool {
        if nums.len() < 3 {
            return false;
        }

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

            if  Solution::two_sum(&nums[i+1..], target-nums[i]) {
                return true;
            }

            i += 1;
        }
        false
    }

    pub fn two_sum(nums: &[i32], target: i32) -> bool {
        let mut result: Vec<(i32, i32)> = Vec::new();
        let (mut i, mut j) = (0_usize, nums.len() - 1);
        while i < j {
            if nums[i] + nums[j] > target {
                j -= 1;
            } else if nums[i] + nums[j] < target {
                i += 1;
            } else {
                return true;
            }
        }
        false
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

mod tests{
    #[test]
    pub fn test_solution() {
        use super::*;
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }
}