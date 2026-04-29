impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();

        let (mut pacific, mut atlantic) = (HashSet::new(), HashSet::new());

        // check pacific
        for i in 0..n {
            if !pacific.contains(&(0, i)) {
                Solution::dfs(&heights, &mut pacific, 0, i, m, n);
            }
        }
        for j in 1..m {
            if !pacific.contains(&(j, 0)) {
                Solution::dfs(&heights, &mut pacific, j, 0, m, n);
            }
        }

        // check atlantic
        for i in 0..n {
            if !atlantic.contains(&(m-1, i)) {
                Solution::dfs(&heights, &mut atlantic, m-1, i, m, n);
            }
        }
        for j in 0..m-1 {
            if !atlantic.contains(&(j, n-1)) {
                Solution::dfs(&heights, &mut atlantic, j, n-1, m, n);
            }    
        }

        let mut result = vec![];
        for key in pacific {
            if atlantic.contains(&key) {
                result.push(vec![key.0 as i32, key.1 as i32]);
            }
        }
        result
    }

    fn dfs(heights: &Vec<Vec<i32>>, visited: &mut HashSet<(usize, usize)>, j: usize, i: usize, m: usize, n: usize) {
        visited.insert((j, i));
        let offset_y = [-1, 0, 1, 0];
        let offset_x = [ 0, 1, 0,-1];
        for k in 0..4 {
            let y = offset_y[k] + j as i32;
            let x = offset_x[k] + i as i32;
            if y >= 0 && y < m as i32 && x >= 0 && x < n as i32 && 
               !visited.contains(&(y as usize, x as usize)) && 
               heights[y as usize][x as usize] >= heights[j][i] {
                Solution::dfs(heights, visited, y as usize, x as usize, m, n);
            }
        }
    }
}
