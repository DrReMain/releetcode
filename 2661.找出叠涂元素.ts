/*
 * @lc app=leetcode.cn id=2661 lang=typescript
 *
 * [2661] 找出叠涂元素
 */

// @lc code=start
function firstCompleteIndex(arr: number[], mat: number[][]): number {
    const [m, n] = [mat.length, mat[0].length];
    const matrix = new Map<number, [number, number]>();
    const rows = Array(m).fill(0);
    const cols = Array(n).fill(0);
    for (let i = 0; i < m; ++i) {
        for (let j = 0; j < n; ++j) {
            matrix.set(mat[i][j], [i, j]);
        }
    }

    let k = 0;
    while (k < m * n) {
        const [i, j] = matrix.get(arr[k])!;
        if (rows[i] + 1 === n || cols[j] + 1 === m) break;
        else {
            rows[i]++;
            cols[j]++;
        }
        k++;
    }
    return k;
};
// @lc code=end

