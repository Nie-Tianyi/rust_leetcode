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

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    reverse_node(None, head)
}

fn reverse_node(prev: Option<Box<ListNode>>, curr: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match curr {
        None => prev,
        Some(node) => {
            let next = node.next;

            reverse_node(
                Some(Box::new(ListNode {
                    val: node.val,
                    next: prev,
                })),
                next,
            )
        }
    }
}
