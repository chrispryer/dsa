from src.tree import TreeNode, is_balanced


def test_tree() -> None:
    tree = TreeNode(val=2, left=TreeNode(val=1), right=TreeNode(val=3))

    assert is_balanced(tree)
