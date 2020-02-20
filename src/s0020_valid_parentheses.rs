use std::collections::HashMap;

struct Solution();

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for elm in s.chars().into_iter() {
            match stack.last() {
                None => {},
                Some(&v) => {
                    if Solution::pair(v, elm) {
                        stack.pop();
                        continue
                    }
                }
            }
            stack.push(elm);
        }

        stack.is_empty()
    }

    pub fn pair(open: char, close: char) -> bool {
        (open == '(' && close == ')')
            || (open == '[' && close == ']')
            || (open == '{' && close == '}')
    }
}

mod tests {
    #[test]
    pub fn test_solution() {
        use super::*;
        assert_eq!(Solution::is_valid(String::from("[](){}")), true);
        assert_eq!(Solution::is_valid(String::from("{([])}")), true);
        assert_eq!(Solution::is_valid(String::from("[()]")), true);
        assert_eq!(Solution::is_valid(String::from("[{)]")), false);
        assert_eq!(Solution::is_valid(String::from("")), true);
    }
}