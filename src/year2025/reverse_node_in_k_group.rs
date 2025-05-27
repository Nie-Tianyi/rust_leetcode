use crate::list_util::ListNode;

struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut result: Option<Box<ListNode>> = None;
        let mut curr = &mut result;
        let mut queue: Vec<Box<ListNode>> = Vec::new();

        'outer: loop {
            for _ in 0..k {
                if let Some(mut node) = head {
                    // 挨个将node push到queue中
                    head = node.next.take();
                    queue.push(node);
                } else {
                    // 如果不足k个，则按原顺序在result链表后加上node
                    for node in queue {
                        curr = &mut curr.insert(node).next;
                    }
                    break 'outer;
                }
            }
            // 如果足够k个，则将node按相反顺序添加到链表后面
            for _ in 0..k {
                curr = &mut curr.insert(queue.pop().unwrap()).next;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked_list;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_k_group(linked_list![1, 2, 3, 4, 5], 2),
            linked_list![2, 1, 4, 3, 5]
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::reverse_k_group(linked_list![1, 2, 3, 4, 5], 3),
            linked_list![3, 2, 1, 4, 5]
        )
    }
}
