
class TreeNode {
    val: number
    left: TreeNode | null
    right: TreeNode | null
    constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.left = (left===undefined ? null : left)
        this.right = (right===undefined ? null : right)
    }
}

function maxPathSum(root: TreeNode | null): number {
    let sum = -Infinity;

    function dfs(_root: TreeNode | null) {
        if(!_root) return 0;

        const left = Math.max(dfs(_root.left), 0)
        const right = Math.max(dfs(_root.right), 0)

        sum = Math.max(sum , _root.val + left + right)

        return _root.val + Math.max(left, right);
    }

    dfs(root)
    return sum;
};