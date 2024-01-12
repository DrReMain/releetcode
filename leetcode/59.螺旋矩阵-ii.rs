/*
 * @lc app=leetcode.cn id=59 lang=rust
 *
 * [59] 螺旋矩阵 II
 */

// @lc code=start
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut ret: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];
        let (mut x, mut y, mut dir) = (0usize, 0usize, 0usize);
        for i in 1..=n*n {
            ret[x][y] = i as i32;
            let (nextX, nextY) = (x as i32+directions[dir].0, y as i32+directions[dir].1);
            if nextX >= n || nextX < 0 ||
                nextY >= n || nextY < 0 ||
                ret[nextX as usize][nextY as usize] != 0 {
                    dir = (dir+1)%directions.len();
                }
            x = (x as i32 + directions[dir].0) as usize;
            y = (y as i32 + directions[dir].1) as usize;
        }
        ret
    }
}
// @lc code=end

