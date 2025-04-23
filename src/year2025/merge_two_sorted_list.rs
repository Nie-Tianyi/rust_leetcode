use crate::list_util::ListNode;

struct Solution;
impl Solution {
    #[allow(unused)]
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;
        let mut res = None;
        let mut res_ptr = &mut res;

        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                let tmp = list1.as_mut().unwrap().next.take();
                if res_ptr.is_none() {
                    *res_ptr = list1;
                } else {
                    res_ptr.as_mut().unwrap().next = list1;
                    res_ptr = &mut res_ptr.as_mut().unwrap().next
                }
                list1 = tmp;
            } else {
                let tmp = list2.as_mut().unwrap().next.take();
                if res_ptr.is_none() {
                    *res_ptr = list2;
                } else {
                    res_ptr.as_mut().unwrap().next = list2;
                    res_ptr = &mut res_ptr.as_mut().unwrap().next
                }
                list2 = tmp
            }
        }

        match (&list1, &list2) {
            (None, Some(_)) => {
                if res_ptr.is_none() {
                    *res_ptr = list2
                } else {
                    res_ptr.as_mut().unwrap().next = list2
                }
            }
            (Some(_), None) => {
                if res_ptr.is_none() {
                    *res_ptr = list1
                } else {
                    res_ptr.as_mut().unwrap().next = list1
                }
            }
            (None, None) => (),
            (Some(_), Some(_)) => unreachable!(),
        }

        res
    }
}

struct Solution2;
impl Solution2 {
    #[allow(unused)]
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut list1, mut list2) = (list1, list2);
        let mut result = None;
        let mut curr = &mut result;

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

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked_list;

    #[test]
    fn test() {
        let l1 = linked_list![1, 2, 4];
        let l2 = linked_list![1, 3, 4];

        assert_eq!(
            Solution::merge_two_lists(l1, l2),
            linked_list![1, 1, 2, 3, 4, 4]
        )
    }
}
