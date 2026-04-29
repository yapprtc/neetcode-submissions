impl Solution {
    const TREASURE:i32 = 0;
    const LAND:i32 = 2147483647;
    const WATER:i32 = -1;
    const OFFSET_Y:[i32; 4] = [-1, 0, 1, 0];
    const OFFSET_X:[i32; 4] = [ 0, 1, 0,-1];

    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        let m = grid.len(); assert!(m>=1);
        let n = grid[0].len(); assert!(n>=1);
        for j in 0..m {
            for i in 0..n {
                if grid[j][i] == Solution::TREASURE{
                    Solution::bfs(grid, j, i, m, n);
                }
            }
        }
    }

    fn bfs(grid: &mut Vec<Vec<i32>>, j: usize, i: usize, m: usize, n: usize) {
        let mut queue = VecDeque::new();
        queue.push_back((j, i));
        while let Some((j, i)) = queue.pop_front() {
            for k in 0..4 {
                let y = Solution::OFFSET_Y[k] + j as i32;
                let x = Solution::OFFSET_X[k] + i as i32;
                if y >= 0 && y < m as i32 && x >= 0 && x < n as i32 && grid[j][i] + 1 < grid[y as usize][x as usize] {
                    grid[y as usize][x as usize] = grid[j][i] + 1;
                    queue.push_back((y as usize, x as usize));
                }
            }
        }
    }
}
