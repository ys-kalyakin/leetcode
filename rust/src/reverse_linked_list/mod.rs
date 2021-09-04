/// https://leetcode.com/problems/reverse-linked-list/

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
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut it = head;

    let mut prev = None;
    while let Some(mut v) = it {
        let mut next = v.next.take();
        v.next = prev.take();
        prev = Some(v);
        it = next.take();
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::ListNode;

    #[test]
    fn reverse_list_test() {
        let lst = ListNode::new_node(
            1,
            ListNode::new_node(
                2,
                ListNode::new_node(3, ListNode::new_node(4, ListNode::new(5))),
            ),
        );
        assert_eq!(vec![1, 2, 3, 4, 5], Some(Box::new(lst)).unwrap().to_vec());
    }
}
