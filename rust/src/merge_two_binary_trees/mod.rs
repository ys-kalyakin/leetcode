use std::cell::RefCell;
use std::rc::Rc;

/// https://leetcode.com/problems/merge-two-binary-trees/
#[allow(dead_code)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    #[allow(dead_code)]
    pub fn new_node(val: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> Self {
        TreeNode {
            val,
            left: if let Some(v) = left {
                Some(Rc::new(RefCell::new(v)))
            } else {
                None
            },
            right: if let Some(v) = right {
                Some(Rc::new(RefCell::new(v)))
            } else {
                None
            },
        }
    }
}

#[allow(dead_code)]
pub fn merge_trees(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn inner(
        r1: &Option<Rc<RefCell<TreeNode>>>,
        r2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (r1, r2) {
            (Some(r1), Some(r2)) => {
                let (l, r) = (r1.borrow(), r2.borrow());
                let mut root = TreeNode::new(l.val + r.val);

                root.left = inner(&l.left, &r.left);
                root.right = inner(&l.right, &r.right);
                Some(Rc::new(RefCell::new(root)))
            },
            (None, Some(r2)) => Some(r2.clone()),
            (Some(r1), None) => Some(r1.clone()),
            _ => None
        }
    }

    return inner(&root1, &root2);
}

#[cfg(test)]
mod tests {
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn merge_trees_test() {
        let root1 = TreeNode::new_node(
            1,
            Some(TreeNode::new_node(3, Some(TreeNode::new(5)), None)),
            Some(TreeNode::new(2)),
        );
        let root2 = TreeNode::new_node(
            2,
            Some(TreeNode::new_node(1, None, Some(TreeNode::new(4)))),
            Some(TreeNode::new_node(3, None, Some(TreeNode::new(7)))),
        );

        super::merge_trees(
            Some(Rc::new(RefCell::new(root1))),
            Some(Rc::new(RefCell::new(root2))),
        );
    }
}
