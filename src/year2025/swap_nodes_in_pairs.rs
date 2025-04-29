use crate::list_util::ListNode;

struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut curr = &mut head;

        loop {
            let mut a = curr.take(); // 取出第一个node

            if a.is_none() {
                break;
            }

            let mut b = a.as_mut().unwrap().next.take();

            if b.is_none() {
                *curr = a;
                break;
            }

            a.as_mut().unwrap().next = b.as_mut().unwrap().next.take(); // b后面的node接到a后面去
            *curr = b; // b接到头上去
            curr.as_mut().unwrap().next = a; // a接到b后面
            curr = &mut curr.as_mut().unwrap().next.as_mut().unwrap().next; //指针往后移动两个node
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::linked_list;
    use crate::list_util::ListNode;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::swap_pairs(linked_list![1, 2, 3, 4]),
            linked_list![2, 1, 4, 3]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::swap_pairs(linked_list![]), linked_list![]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::swap_pairs(linked_list![1]), linked_list![1]);
    }
}
