// Definition for a binary tree node.

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q1 = VecDeque::new();
        let mut q2 = VecDeque::new();
        let mut result: Vec<Vec<i32>> = Vec::new();

        if root.is_some() {
            q1.push_back(root.unwrap());
            while !q1.is_empty() || !q2.is_empty() {
                let mut temp = Solution::traversal(&mut q1, &mut q2);
                if !temp.is_empty() {
                    result.push(temp);
                }

                let mut temp: Vec<i32> = Solution::traversal(&mut q2, &mut q1);
                if !temp.is_empty() {
                    result.push(temp);
                }
            }
        } else {
            return vec![]
        }

        result
    }

    fn traversal(origin: &mut VecDeque<Rc<RefCell<TreeNode>>>,
                 target: &mut VecDeque<Rc<RefCell<TreeNode>>>
    ) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        while !origin.is_empty() {
            let node = origin.pop_front().unwrap();
            result.push(node.borrow().val);
            node.borrow().left.as_ref().map(|n| target.push_back(n.clone()));
            node.borrow().right.as_ref().map(|n| target.push_back(n.clone()));
        }
        result
    }
}

mod tests {
    #[test]
    pub fn test_solution() {

    }
}