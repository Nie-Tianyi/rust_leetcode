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

/// 请在在一个链表中每相邻的两个节点间插入一个结点, 插入节点的val为两个节点val的最大公约数
/// 例如： [10, 5, 6] -> [10, 5, 5, 1, 6]
///                          ^     ^
/// 复杂度要求： O(n)/O(1)
#[allow(dead_code)]
pub fn insert_greatest_common_divisors(
    head: &mut Option<Box<ListNode>>,
) -> &mut Option<Box<ListNode>> {
    // 初始化指针指向头节点
    let mut node = head.as_mut();

    while let Some(n) = node {
        // 如果有下一个节点
        if n.next.is_some() {
            // 求这个节点和下一个节点value的最大公因数
            let gcd = greatest_common_divisor(n.val, n.next.as_ref().unwrap().val);
            // 插入节点
            insert_node(&mut n.next, gcd);
            // 将指针移动两下
            node = n.next.as_mut().unwrap().next.as_mut();
        } else {
            break;
        }
    }

    head
}

// 在给定节点后插入一个新的节点
fn insert_node(node: &mut Option<Box<ListNode>>, value: i32) -> &mut Option<Box<ListNode>> {
    // 如果是node是None，直接返回
    // 如果不是
    if let Some(n) = node.as_mut() {
        // 将下一个节点暂时储存起来
        let next_node = n.next.take();
        // 新建中间的节点
        let new_node = Some(Box::new(ListNode {
            val: value,
            next: next_node,
        }));
        // 将当前节点和下一个节点连接起来
        n.next = new_node;
    }
    node
}

// 短除法求最大公因数
fn greatest_common_divisor(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        greatest_common_divisor(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        let gcd = greatest_common_divisor(3_i32, 12_i32);
        assert_eq!(gcd, 3_i32)
    }
}
