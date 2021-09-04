/// https://leetcode.com/problems/middle-of-the-linked-list/

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
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut size = 0;

    let mut it = &head;
    while let Some(v) = it {
        size += 1;
        it = &v.next;
    }

    it = &head;
    let mut i = 0;
    while let Some(v) = it {
        if i == size / 2 {
            return (*it).clone();
        }
        i += 1;
        it = &v.next;
    }

    None
}

#[allow(dead_code)]
pub fn middle_node_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    (*slow).clone()
}

#[cfg(test)]
mod tests {
    #[test]
    fn middle_node_test() {
        let lst = super::ListNode::new_node(
            1,
            super::ListNode::new_node(
                2,
                super::ListNode::new_node(3, super::ListNode::new_node(4, super::ListNode::new(5))),
            ),
        );
        assert_eq!(
            vec![3, 4, 5],
            super::middle_node(Some(Box::new(lst))).unwrap().to_vec()
        );
    }

    #[test]
    fn middle_node_2_test() {
        let lst = super::ListNode::new_node(
            1,
            super::ListNode::new_node(
                2,
                super::ListNode::new_node(3, super::ListNode::new_node(4, super::ListNode::new(5))),
            ),
        );
        assert_eq!(
            vec![3, 4, 5],
            super::middle_node_2(Some(Box::new(lst))).unwrap().to_vec()
        );
    }
}
