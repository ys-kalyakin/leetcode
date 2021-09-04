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
pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result = Some(Box::new(ListNode::new(0)));
    let mut h = &mut result;

    let mut first_iter = &l1;
    let mut second_iter = &l2;

    while let (Some(x), Some(y)) = (first_iter, second_iter) {
        if x.val < y.val {
            h.as_mut().unwrap().next = Some(Box::new(ListNode::new(x.val)));
            h = &mut h.as_mut().unwrap().next;
            first_iter = &x.next;
        } else {
            h.as_mut().unwrap().next = Some(Box::new(ListNode::new(y.val)));
            h = &mut h.as_mut().unwrap().next;
            second_iter = &y.next;
        }
    }
    if let Some(_) = first_iter {
        while let Some(x) = first_iter {
            h.as_mut().unwrap().next = Some(Box::new(ListNode::new(x.val)));
            h = &mut h.as_mut().unwrap().next;
            first_iter = &x.next;
        }
    } else {
        while let Some(y) = second_iter {
            h.as_mut().unwrap().next = Some(Box::new(ListNode::new(y.val)));
            h = &mut h.as_mut().unwrap().next;
            second_iter = &y.next;
        }
    }
    return result.unwrap().next;
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    #[test]
    fn merge_two_lists_test() {
        let lst1 = ListNode::new_node(1, ListNode::new_node(2, ListNode::new(4)));
        let lst2 = ListNode::new_node(1, ListNode::new_node(3, ListNode::new(4)));
        assert_eq!(
            vec![1, 1, 2, 3, 4, 4],
            super::merge_two_lists(Some(Box::new(lst1)), Some(Box::new(lst2)))
                .unwrap()
                .to_vec()
        );
    }
}
