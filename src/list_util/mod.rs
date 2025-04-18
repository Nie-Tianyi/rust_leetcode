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

#[allow(unused_macros)]
#[macro_export]
macro_rules! linked_list {
    // 处理空链表的情况
    () => {
        None
    };
    // 递归分解元素，第一个元素作为当前节点，剩余元素生成后续链表
    ($elem:expr $(, $rest:expr)*) => {
        Some(Box::new(ListNode {
            val: $elem,
            next: linked_list!($($rest),*)
        }))
    };
}
