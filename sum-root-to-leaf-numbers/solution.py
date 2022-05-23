# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


def is_leaf_node(node: TreeNode):
    return node.left is None and node.right is None


def find_paths_of_leaves(path_so_far: List[int], node: TreeNode) -> List[List[int]]:
    print(f"path: {path_so_far} / node: {node}")
    # each time we get a node, we can see if we have leaves
    complete_paths = []
    next_path_so_far = path_so_far + [node.val]

    if node.left is not None:
        print(f"checking left node: {node.left}")
        if is_leaf_node(node.left):
            print(f"found leaf node: {node.right}")
            complete_paths.append(path_so_far + [node.val, node.left.val])
        else:
            print("not leaf node")
            complete_paths += find_paths_of_leaves(
                next_path_so_far, node.left
            )  # keep going

    if node.right is not None:
        print(f"checking right node: {node.right}")
        if is_leaf_node(node.right):
            print(f"found leaf node: {node.right}")
            complete_paths.append(path_so_far + [node.val, node.right.val])
        else:
            print("not leaf node)")
            complete_paths += find_paths_of_leaves(
                next_path_so_far, node.right
            )  # keep going

    return complete_paths


def path_to_sum(path: List[int]):
    digits = [str(digit) for digit in path]
    string = "".join(digits)  # this syntax is so annoying
    return int(string)


class Solution:
    def sumNumbers(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        if is_leaf_node(root):
            return root.val

        paths: List[List[int]] = find_paths_of_leaves([], root)

        print(paths)

        return sum([path_to_sum(path) for path in paths])
