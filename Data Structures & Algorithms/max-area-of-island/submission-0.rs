impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len(); assert!(m>=1);
        let n = grid[0].len(); assert!(n>=1);
        let mut visited = vec![];
        for _ in 0..m {
            visited.push(vec![false; n]);
        }
        let mut global = 0;
        for j in 0..m {
            for i in 0..n {
                if !visited[j][i] && grid[j][i] == 1 {
                    let mut local = 0;
                    Solution::dfs(&grid, &mut visited, &mut local, j, i, m, n);
                    global = std::cmp::max(global, local);
                }
            }
        }
        global
    }

    fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, local: &mut i32, j: usize, i: usize, m: usize, n: usize) {
        visited[j][i] = true;
        *local += 1;
        const offset_y:[i32; 4] = [-1, 0, 1, 0];
        const offset_x:[i32; 4] = [0, 1, 0, -1];
        for k in 0..4 {
            let y = offset_y[k] + j as i32;
            let x = offset_x[k] + i as i32;
            if y >=0 && y < m as i32 && x >= 0 && x < n as i32 && !visited[y as usize][x as usize] && grid[y as usize][x as usize] == 1 {
                Solution::dfs(grid, visited, local, y as usize, x as usize, m, n);
            }
        }
    }
}
