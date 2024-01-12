/*
 * @lc app=leetcode.cn id=2661 lang=rust
 *
 * [2661] 找出叠涂元素
 */

// @lc code=start
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let (m, n) = (mat.len(), mat[0].len());
        let mut matrix: HashMap<i32, (usize, usize)> = HashMap::new();
        let mut rows = vec![0; m];
        let mut cols = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                matrix.insert(mat[i][j], (i, j));
            }
        }

        let mut k = 0usize;
        while k < m * n {
            let (i, j) = matrix[&arr[k]];
            if rows[i] + 1 == n || cols[j] + 1 == m {
                break;
            } else {
                rows[i] += 1;
                cols[j] += 1;
            }
            k += 1;
        }
        k as i32
    }
}
// @lc code=end

