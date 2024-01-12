/*
 * @lc app=leetcode.cn id=490 lang=typescript
 *
 * [490] 迷宫
 */

// @lc code=start
function hasPath(maze: number[][], start: number[], destination: number[]): boolean {
    const [stack, dirs, set] = [
        [start],
        [[-1, 0], [1, 0], [0, 1], [0, -1]],
        new Set([start.join("#")]),
    ];
    while (stack.length) {
        const [x, y] = stack.pop()!;
        if (x === destination[0] && y === destination[1]) return true;

        for (let [dx, dy] of dirs) {
            let [i, j] = [x + dx, y + dy];
            while (maze?.[i]?.[j] === 0) {
                i += dx;
                j += dy;
            }
            i -= dx, j -= dy;
            if (!set.has([i, j].join("#"))) {
                stack.push([i, j]);
                set.add([i, j].join("#"));
            }
        }
    }
    return false;
};
// @lc code=end

