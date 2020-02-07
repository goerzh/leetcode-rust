use std::collections::HashMap;

struct Solution();

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }

        let mut mapping: HashMap<char, char> = HashMap::new();
        mapping.insert('(', ')');
        mapping.insert('[', ']');
        mapping.insert('{', '}');

        let mut left = Vec::new();
        for ref elm in s.chars() {
            if mapping.contains_key(elm) {
                left.push(elm.clone());
            } else {
                match left.pop().and_then(
                    |c| mapping.get(&c).map(|c| c == elm)
                ) {
                    Some(v) => {
                        if !v {
                            return false;
                        }
                    },
                    None => { return false }
                }
            }
        }

        return left.len() == 0;
    }
}

mod tests {
    #[test]
    pub fn test_l20() {
        use super::*;
        assert_eq!(Solution::is_valid(String::from("[](){}")), true);
        assert_eq!(Solution::is_valid(String::from("{([])}")), true);
        assert_eq!(Solution::is_valid(String::from("[()]")), true);
        assert_eq!(Solution::is_valid(String::from("[{)]")), false);
        assert_eq!(Solution::is_valid(String::from("")), true);
    }
}