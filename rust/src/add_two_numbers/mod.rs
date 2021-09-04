/// https://leetcode.com/problems/add-two-numbers/

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
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
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1_current = l1;
    let mut l2_current = l2;
    let mut result = Some(Box::new(ListNode::new(0)));
    let mut current_node = result.as_mut();
    let mut carry = 0;

    while l1_current.is_some() || l2_current.is_some() {
        let mut current_sum = carry;

        if let Some(node) = l1_current {
            current_sum += node.val;
            l1_current = node.next;
        }

        if let Some(node) = l2_current {
            current_sum += node.val;
            l2_current = node.next;
        }

        carry = current_sum / 10;

        if let Some(n) = current_node {
            n.next = Some(Box::new(ListNode::new(current_sum % 10)));
            current_node = n.next.as_mut();
        }
    }

    if carry > 0 {
        current_node.unwrap().next = Some(Box::new(ListNode::new(carry)));
    }

    result.unwrap().next
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_two_numbers_test() {
        let lst1 =
            super::ListNode::new_node(2, super::ListNode::new_node(4, super::ListNode::new(9)));
        let lst2 = super::ListNode::new_node(
            1,
            super::ListNode::new_node(9, super::ListNode::new_node(9, super::ListNode::new(9))),
        );
        let res = super::add_two_numbers(Some(Box::new(lst1)), Some(Box::new(lst2)))
            .unwrap()
            .to_vec();
        assert_eq!(vec![3, 3, 9, 0, 1], res);
    }
}
