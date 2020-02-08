use std::collections::{HashMap, HashSet, BTreeSet};

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for (i, elm) in nums.iter().enumerate() {
            let mut temp = Solution::two_sum(nums[i+1..].iter().cloned().collect(), 0-elm.clone());
            for v in temp.iter_mut() {
                v.push(elm.clone());
            }
            result.extend(temp);
        }
        result = Solution::sort(result);
        result = Solution::dedup(result);

        result
    }

    pub fn dedup(mut input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut temp: HashSet<Vec<i32>> = HashSet::new();
        let mut result: Vec<Vec<i32>> = Vec::new();
        for elm in input.into_iter() {
            temp.insert(elm);
        }

        for elm in temp.into_iter() {
            result.push(elm);
        }
        result
    }

    pub fn sort(mut input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for v in input.iter_mut() {
            v.sort()
        }
        input
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut pair: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<Vec<i32>> = Vec::new();
        for elm in nums.iter() {
            if pair.contains_key(elm) {
                let mut temp: Vec<i32> = Vec::new();
                temp.push(elm.clone());
                temp.push(pair.get(elm).unwrap().clone());
                result.push(temp);
            } else {
                pair.insert(target - elm.clone(), elm.clone());
            }
        }
        result
    }
}


mod tests {
    #[test]
    pub fn test_solution() {
        use super::*;
        assert!(vec![1,2,3]==vec![1,2,3]);
        assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
                   vec![vec![-1, 0, 1], vec![-1, -1, 2]]);
    }
}