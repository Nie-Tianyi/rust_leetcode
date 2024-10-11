pub struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two(l1, l2, 0)
    }

    fn add_two(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => {
                if carry != 0 {
                    Some(Box::new(ListNode::new(carry)))
                } else {
                    None
                }
            }
            (Some(node1), None) => {
                if carry != 0 {
                    let sum = node1.val + carry;
                    Some(Box::new(ListNode {
                        val: sum % 10,
                        next: Self::add_two(node1.next, None, sum / 10),
                    }))
                } else {
                    Some(node1)
                }
            }
            (None, Some(node2)) => {
                if carry != 0 {
                    let sum = node2.val + carry;
                    Some(Box::new(ListNode {
                        val: sum % 10,
                        next: Self::add_two(None, node2.next, sum / 10),
                    }))
                } else {
                    Some(node2)
                }
            }
            (Some(node1), Some(node2)) => {
                let res = node1.val + node2.val + carry;
                Some(Box::new(ListNode {
                    val: res % 10,
                    next: Self::add_two(node1.next, node2.next, res / 10),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let l1 = ListNode{
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None,
                }))
            }))
        };

        let l2 = ListNode{
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                }))
            }))
        };

        assert_eq!(
            Solution::add_two_numbers(Some(Box::new(l1)),Some(Box::new(l2))),
            Some(Box::new(ListNode{
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 8,
                        next: None,
                    }))
                }))
            }))
        )
    }
}
