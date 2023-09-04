class TreeNode {
    val: number
    left: TreeNode | null
    right: TreeNode | null
    constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.left = (left === undefined ? null : left)
        this.right = (right === undefined ? null : right)
    }
}

function serialize(root: TreeNode | null): string {
    const list = [];

    const iter = (_root: TreeNode | null, _list) => {
        if (!_root) return

        iter(_root.left, _list);
        iter(_root.right, _list);
        _list.push(_root.val);
    }

    iter(root, list);
    return list.join('-');
};

function deserialize(data: string): TreeNode | null {
    if (!data.length) return null;

    let list = data.split('-').map(_ => parseInt(_));

    const construct = (lower: number, upper: number, _list: number[]) => {
        if (!_list.length || _list[_list.length - 1] < lower || _list[_list.length - 1] > upper) return null;

        const value = _list.pop();
        const root = new TreeNode(value);
        root.right = construct(value, upper, _list);
        root.left = construct(lower, value, _list);
        return root;
    }

    return construct(-Number.MAX_SAFE_INTEGER, Number.MAX_SAFE_INTEGER, list);
};