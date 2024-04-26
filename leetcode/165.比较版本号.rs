/*
 * @lc app=leetcode.cn id=165 lang=rust
 *
 * [165] 比较版本号
 */

// @lc code=start
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let (version1, version2) = (
            version1.as_bytes(),
            version2.as_bytes(),
        );
        let (mut x, mut y) = (0, 0);
        while x < version1.len() || y < version2.len() {
            let (mut n1, mut n2) = (0, 0);
            while x < version1.len() && version1[x] != b'.' {
                n1 = n1 * 10 + (version1[x] - b'0') as i32;
                x += 1;
            }
            while y < version2.len() && version2[y] != b'.' {
                n2 = n2 * 10 + (version2[y] - b'0') as i32;
                y += 1;
            }
            if n1 > n2 {
                return 1;
            } else if n1 < n2 {
                return -1;
            }

            x += 1;
            y += 1;
        }
        0
    }
}
// @lc code=end

