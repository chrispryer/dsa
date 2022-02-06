use dsa::tree::{is_balanced_bst, TreeNode};

#[test]
fn test_tree() {
    let mut tree = TreeNode::new(2);
    tree.insert(1);
    tree.insert(3);

    assert!(is_balanced_bst(&tree));
}
