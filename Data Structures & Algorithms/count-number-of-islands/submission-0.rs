impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len(); assert!(m!=0);
        let n = grid[0].len(); assert!(n!=0);

        let mut visited = vec![];
        for _ in 0..m {
            visited.push(vec![false; n]);
        }

        let mut cc = 0;
        for j in 0..m {
            for i in 0..n {
                if !visited[j][i] && grid[j][i] == '1' {
                    Solution::dfs(&mut visited, &grid, i, j, m, n);
                    cc += 1;
                }
            }
        }
        cc
    }

    fn dfs(visited: &mut Vec<Vec<bool>>, grid: &Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
        visited[j][i] = true;
        let offset_y = [-1, 0, 1, 0];
        let offset_x = [ 0, 1, 0,-1]; 
        for k in 0..4 {
            let y: i32 = offset_y[k] + j as i32;
            let x: i32 = offset_x[k] + i as i32;
            if y >= 0 && y < m as i32 && x >= 0 && x < n as i32 && !visited[y as usize][x as usize] && grid[y as usize][x as usize] == '1' {
                Solution::dfs(visited, grid, x as usize, y as usize, m, n);
            }
        }
    }
}
