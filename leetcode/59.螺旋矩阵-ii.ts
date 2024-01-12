/*
 * @lc app=leetcode.cn id=59 lang=typescript
 *
 * [59] 螺旋矩阵 II
 */

// @lc code=start
function generateMatrix(n: number): number[][] {
    const directions = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    const ret: number[][] = Array(n).fill([]);
    for (let i in ret) ret[i] = Array(n).fill(0);
    let [x, y, dir] = [0, 0, 0];
    for (let i = 1; i <= n*n; i++) {
        ret[x][y] = i;
        const [nextX, nextY] = [x+directions[dir][0], y+directions[dir][1]];
        if (
            nextX >= n || nextX < 0 ||
            nextY >= n || nextY < 0 ||
            ret[nextX][nextY] !== 0
        ) dir = (dir+1)%directions.length;
        x += directions[dir][0];
        y += directions[dir][1];
    }
    return ret;
};
// @lc code=end

