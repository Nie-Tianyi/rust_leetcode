use crate::list_util::ListNode;

struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut lists = lists;
        let mut res = lists[0].take();
        for i in 1..lists.len() {
            res = merge_two_list(res, lists[i].take());
        }

        res
    }
}

#[inline]
fn merge_two_list(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut curr = &mut head;

    *curr = loop {
        match (list1, list2) {
            (Some(mut a), Some(mut b)) => {
                if a.val <= b.val {
                    list1 = a.next.take();
                    list2 = Some(b);
                    curr = &mut curr.insert(a).next;
                } else {
                    list1 = Some(a);
                    list2 = b.next.take();
                    curr = &mut curr.insert(b).next;
                }
            }
            (a, b) => break a.or(b),
        }
    };

    head
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::linked_list;
    use crate::list_util::ListNode;

    #[test]
    fn test_1() {
        let ques = [
            linked_list![1, 4, 5],
            linked_list![2, 3, 4],
            linked_list![2, 6],
        ];
        let expected = linked_list![1, 2, 2, 3, 4, 4, 5, 6];
        assert_eq!(Solution::merge_k_lists(ques.to_vec()), expected)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::merge_k_lists(vec![]), linked_list![]);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::merge_k_lists(vec![linked_list![]]),
            linked_list![]
        );
    }
}
