/*
 * @lc app=leetcode.cn id=490 lang=rust
 *
 * [490] 迷宫
 */

// @lc code=start
impl Solution {
    pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let (row, col) = (maze.len(), maze[0].len());
        let (mut stack, dirs, mut set) = (
            vec![start.clone()],
            vec![(-1, 0), (1, 0), (0, 1), (0, -1)],
            vec![format!("{}#{}", start[0], start[1])]
                .into_iter()
                .collect::<HashSet<String>>(),
        );
        while stack.len() > 0 {
            let block = stack.pop().unwrap();
            let (x, y) = (block[0], block[1]);
            if x == destination[0] && y == destination[1] {
                return true;
            }

            for d in dirs.iter() {
                let (mut i, mut j) = (x + d.0, y + d.1);
                while i >= 0
                    && j >= 0
                    && i < row as i32
                    && j < col as i32
                    && maze[i as usize][j as usize] == 0
                {
                    i += d.0;
                    j += d.1;
                }
                i -= d.0;
                j -= d.1;

                if !set.contains(&format!("{}#{}", i, j)) {
                    stack.push(vec![i, j]);
                    set.insert(format!("{}#{}", i, j).to_string());
                }
            }
        }
        false
    }
}
// @lc code=end

