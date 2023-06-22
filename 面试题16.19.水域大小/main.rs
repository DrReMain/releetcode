impl Solution {
    pub fn pond_sizes(land: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let mut land = land.clone();

        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if land[i][j] == 0 {
                    result.push(Self::dfs(&mut land, i, j));
                }
            }
        }

        result.sort();
        result
    }

    fn dfs(land: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
        land[x][y] = 1;
        let mut size = 1;

        let t = [1,0,-1,0,1,1,-1,-1,1];
        for i in 0..8 {
            let nx = x as i32 + t[i];
            let ny = y as i32 + t[i + 1];
            if Self::in_area(land, nx, ny) && land[nx as usize][ny as usize] == 0 {
                size += Self::dfs(land, nx as usize, ny as usize);
            }
        }

        size
    }

    fn in_area(land: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
        i >= 0 && i < land.len() as i32 && j >= 0 && j < land[0].len() as i32
    }
}
