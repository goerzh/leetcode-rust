use std::borrow::BorrowMut;

// Definition for singly-linked list.
 #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut carry = false;
        let mut target = 0;

        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode{val:0, next:None}));
        let mut curr = &mut head;
        
        while l1.is_some() || l2.is_some() || carry == true {
            let lhs = match l1 {
                None => {
                    l1 = None;
                    0
                },
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
            };

            let rhs = match l2 {
                None => {
                    l2 = None;
                    0
                },
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
            };
            target = lhs + rhs;

            if carry {
                target += 1;
                carry = false;
            }
            if target >= 10 {
                carry = true;
            }

            curr.as_mut().unwrap().next = Some(Box::new(ListNode{val:target%10, next:None}));
            curr = &mut curr.as_mut().unwrap().next;
        }
        head.unwrap().next
    }
}

mod tests {
    use crate::s0002_add_two_numbers::ListNode;

    #[test]
    pub fn test_solution() {
        use super::*;

        let mut l1 = Some(Box::new(ListNode{
            val: 2,
            next: Some(Box::new(ListNode{
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None,
                }))
            }))
        }));

        let mut l2 = Some(Box::new(ListNode{
            val: 5,
            next: Some(Box::new(ListNode{
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                }))
            }))
        }));

        print_list(Solution::add_two_numbers(l1, l2));
    }

    pub fn print_list(l: Option<Box<ListNode>>) {
        let mut l = l;
        while let Some(v) = l {
            println!("{:?}", v.val);
            l = v.next;
        }
    }
}