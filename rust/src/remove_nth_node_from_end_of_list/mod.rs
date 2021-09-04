/// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

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

    #[allow(dead_code)]
    fn new_node(val: i32, next: ListNode) -> Self {
        ListNode {
            next: Some(Box::new(next)),
            val,
        }
    }

    #[allow(dead_code)]
    fn to_vec(self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = Some(Box::new(self));

        while current.is_some() {
            if let Some(n) = current {
                result.push(n.val);
                current = n.next;
            }
        }
        result
    }
}

#[allow(dead_code)]
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut size = 0;

    let mut it = &head;
    while let Some(v) = it {
        size += 1;
        it = &v.next;
    }

    let mut hc = head.clone();
    let mut h = &mut hc;
    for _ in 0..size - n {
        h = &mut h.as_mut().unwrap().next;
    }
    *h = h.take().unwrap().next;
    hc
}

#[cfg(test)]
mod tests {
    #[test]
    fn remove_nth_from_end_test() {
        let lst = super::ListNode::new_node(
            1,
            super::ListNode::new_node(
                2,
                super::ListNode::new_node(3, super::ListNode::new_node(4, super::ListNode::new(5))),
            ),
        );
        assert_eq!(
            vec![1, 2, 3, 5],
            super::remove_nth_from_end(Some(Box::new(lst)), 2)
                .unwrap()
                .to_vec()
        );
    }
}
