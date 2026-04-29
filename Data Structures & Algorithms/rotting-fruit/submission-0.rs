impl Solution {
    const OFFSET_Y:[i32; 4] = [-1, 0, 1, 0];
    const OFFSET_X:[i32; 4] = [ 0, 1, 0,-1];

    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len(); assert!(m>=1);
        let n = grid[0].len(); assert!(n>=1);
        let mut queue = VecDeque::new();

        let (mut fresh, mut last) = (0, 0);
        for j in 0..m {
            for i in 0..n {
                if grid[j][i] == 1 {
                    fresh += 1;
                } else if grid[j][i] == 2 {
                    queue.push_back((1, j, i));
                }
            }
        }

        while let Some((t, j, i)) = queue.pop_front() {
            for k in 0..4 {
                let y = Solution::OFFSET_Y[k] + j as i32;
                let x = Solution::OFFSET_X[k] + i as i32;
                if y >= 0 && y < m as i32 && x >= 0 && x < n as i32 && grid[y as usize][x as usize] == 1 {
                    grid[y as usize][x as usize] = 2;
                    fresh -= 1;
                    if last != t {
                        last = t;
                    }
                    queue.push_back((t+1, y as usize, x as usize));
                }
            }
        }

        if fresh != 0 {
            -1
        } else {
            last
        }
    }
}
