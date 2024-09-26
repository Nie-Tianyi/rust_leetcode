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
    // // if the head is referring to None, return None directly
    // let first_node = match head{
    //     Some(head_node) => head_node,
    //     None => return None,
    // };
    // // if the second node is None, return first node directly
    // let second_node = match first_node.next {
    //     None => {
    //         return Some(first_node);
    //     },
    //     Some(second_node) => {second_node}
    // };
    // let next_node = second_node;
    // // if the first 2 nodes are not empty
    // loop {
    //     // record the address of next node (original order) first
    //     let next_node = match next_node {
    //         Some(_) => _,
    //         None => None,
    //     };
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(1, 1);
    }
}
