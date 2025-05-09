use crate::list_util::ListNode;

struct Solution;
impl Solution {
    #[allow(unused)]
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let len = count_length(&head);

        // 如果倒数第n个node比整个链表还长，则直接返回
        // 如果n等于0，则直接返回
        if n > len || n == 0 {
            return head;
        }
        let index = len - n;

        let mut head = head;

        if index == 0 {
            head = head.unwrap().next
        } else {
            let mut prev = &mut head;
            for _ in 2..=index {
                prev = &mut prev.as_mut().unwrap().next
            }
            let curr = prev.as_mut().unwrap().next.take();
            let next = curr.unwrap().next;
            prev.as_mut().unwrap().next = next;
        }

        head
    }
}

fn count_length(head: &Option<Box<ListNode>>) -> i32 {
    let mut ptr = head;
    let mut len = 0;

    while ptr.is_some() {
        ptr = &ptr.as_ref().unwrap().next;
        len += 1;
    }

    len
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked_list;

    #[test]
    fn test() {
        let test_list = linked_list![1, 2, 3, 4, 5];

        let expected = linked_list![1, 2, 3, 5];

        let ans = Solution::remove_nth_from_end(test_list, 2);
        assert_eq!(ans, expected)
    }
}
