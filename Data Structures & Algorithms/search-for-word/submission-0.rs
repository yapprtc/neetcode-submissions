impl Solution {
    const OFFSET_Y:[i32; 4] = [-1, 0, 1, 0];
    const OFFSET_X:[i32; 4] = [ 0, 1, 0,-1];

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len(); assert!(m>0);
        let n = board[0].len(); assert!(n>0);
        let mut visited = vec![];
        for _ in 0..m {
            visited.push(vec![false; n]);
        }
        let word = word.as_bytes();

        assert!(!word.is_empty());
        for j in 0..m {
            for i in 0..n {
                if board[j][i] as u8 == word[0] && Solution::dfs(&board, &word, 0, &mut visited, j, i, m, n) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(board:&Vec<Vec<char>>, word:&[u8], d: usize, visited:&mut Vec<Vec<bool>>, j: usize, i: usize, m: usize, n: usize) -> bool {
        if d == word.len() - 1 {
            return true;
        } else if d+1 < word.len() {
            visited[j][i] = true;
            for k in 0..4 {
                let y = Solution::OFFSET_Y[k] + j as i32;
                let x = Solution::OFFSET_X[k] + i as i32;
                if y >= 0 && y < m as i32 && x >= 0 && x < n as i32 &&
                    !visited[y as usize][x as usize] && board[y as usize][x as usize] as u8 == word[d+1] && 
                    Solution::dfs(board, word, d+1, visited, y as usize, x as usize, m, n) {
                    return true;
                }
            }
            visited[j][i] = false;
        }

        false
    }
}
