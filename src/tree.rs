pub struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, val: i32) {
        if self.val == val {
            return;
        }

        let target_node = if val < self.val {
            &mut self.left
        } else {
            &mut self.right
        };

        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(val),
            &mut None => {
                let new_node = TreeNode {
                    val: val,
                    left: None,
                    right: None,
                };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}

pub fn is_balanced(tree: &TreeNode) -> bool {
    if tree.left.is_none() || tree.right.is_none() {
        return true;
    }

    if tree.left.is_none() && !tree.right.is_none() {
        return false;
    }

    if !tree.left.is_none() && tree.right.is_none() {
        return false;
    }

    let left = tree.left.as_ref().unwrap();
    let right = tree.right.as_ref().unwrap();

    if tree.val < left.val || tree.val > right.val {
        return false;
    }

    is_balanced(left) && is_balanced(right)
}
