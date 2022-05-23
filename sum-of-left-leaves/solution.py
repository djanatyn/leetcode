# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def sumOfLeftLeaves(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        left_sum = 0
        right_sum = 0

        # first, check if we have a left node
        if root.left is not None:
            # is the left node a leaf node?
            if root.left.left is None and root.left.right is None:
                left_sum += root.left.val
            # if not, descend down it
            else:
                left_sum += self.sumOfLeftLeaves(root.left)

        # descend down the right node if it exists too
        if root.right is not None:
            right_sum += self.sumOfLeftLeaves(root.right)

        return left_sum + right_sum
