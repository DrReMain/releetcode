/*
 * @lc app=leetcode.cn id=54 lang=typescript
 *
 * [54] 螺旋矩阵
 */

// @lc code=start
function spiralOrder(matrix: number[][]): number[] {
    const [m, n] = [matrix.length, matrix[0].length];
    const directions = [[0, 1], [1, 0], [0, -1], [-1, 0]];

    const visited = Array(m).fill(null).map(() => Array(n).fill(false));
    const ret = Array(m*n).fill(0);
    let [row, col, d] = [0, 0, 0];
    
    for (let i = 0; i < ret.length; ++i) {
        ret[i] = matrix[row][col];
        visited[row][col] = true;
        const [nextRow, nextCol] = [row+directions[d][0], col+directions[d][1]];
        if (
            nextRow < 0 || nextRow >= m ||
            nextCol < 0 || nextCol >= n ||
            visited[nextRow][nextCol]
        ) d = (d+1)%directions.length;

        row += directions[d][0];
        col += directions[d][1];
    }
    return ret; 
};
// @lc code=end

