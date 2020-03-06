struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let (mut p, mut r) = (0, (nums.len()-1) as i32);
        let mut result = 0;
        result = Solution::partition(&mut nums, p, r);
        loop {
            if result + 1 == k  {
                break;
            }
            if result < k {
                p = result + 1;
                result = Solution::partition(&mut nums, p, r);
            } else {
                r = result - 1;
                result = Solution::partition(&mut nums, p, r);
            }
        }

        nums[result as usize]
    }

    pub fn partition(v: &mut [i32], p: i32, r: i32) -> i32 {
        let (mut i, mut j) = (p as usize, p as usize);
        let pivot = v[r as usize];
        while j < r as usize{
            if v[j] > pivot {
                let temp = v[i];
                v[i] = v[j];
                v[j] = temp;

                i += 1;
            }
            j += 1;
        }

        v[r as usize] = v[i];
        v[i] = pivot;

        i as i32
    }
}

mod test{
    use super::*;
    use std::cmp::Reverse;

    #[test]
    pub fn test_solution() {
        assert_eq!(Solution::find_kth_largest(vec![3,2,1,5,6,4], 2), 5);
        assert_eq!(Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4), 4);
        assert_eq!(Solution::find_kth_largest(vec![99, 99], 1), 99);
        assert_eq!(Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6,7,7,8,2,3,1,1,1,10,11,5,6,2,4,7,8,5,6], 20), 2);

    }
}

