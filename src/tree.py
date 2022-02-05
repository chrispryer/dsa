from typing import Optional


class TreeNode:
    def __init__(
        self,
        val: int,
        left: Optional["TreeNode"] = None,
        right: Optional["TreeNode"] = None,
    ) -> "TreeNode":
        self.val = val
        self.left = left
        self.right = right


def is_balanced(tree: TreeNode) -> bool:
    if not tree.left and not tree.right:
        return True

    if not tree.left and tree.right:
        return False

    if tree.left and not tree.right:
        return False

    if tree.val < tree.left.val or tree.val > tree.right.val:
        return False

    return is_balanced(tree=tree.left) and is_balanced(tree=tree.right)
