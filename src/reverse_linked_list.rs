/// https://leetcode.cn/problems/reverse-linked-list/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_list(_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(1, 1);
    }
}
