impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        const OFFSET_Y:[i32; 4] = [-1, 0, 1, 0];
        const OFFSET_X:[i32; 4] = [ 0, 1, 0,-1];

        let m = board.len(); assert!(m>=1);
        let n = board[0].len(); assert!(n>=1);
        let mut queue = VecDeque::new();
        for j in 0..m {
            if board[j][0] == 'O' {
                board[j][0] = 'Z';
                queue.push_back((j, 0));
            }
            if n-1 != 0 && board[j][n-1] == 'O' {
                board[j][n-1] = 'Z';
                queue.push_back((j, n-1));
            }
        }
        for i in 1..n-1 {
            if board[0][i] == 'O' {
                board[0][i] = 'Z';
                queue.push_back((0, i));
            }
            if m-1 != 0 && board[m-1][i] == 'O' {
                board[m-1][i] = 'Z';
                queue.push_back((m-1, i));
            }
        }

        while let Some((j, i)) = queue.pop_front() {
            for k in 0..4 {
                let y = OFFSET_Y[k] + j as i32;
                let x = OFFSET_X[k] + i as i32;
                if y >= 0 && y < m as i32 && x >=0 && x < n as i32 && board[y as usize][x as usize] == 'O' {
                    board[y as usize][x as usize] = 'Z';
                    queue.push_back((y as usize, x as usize));
                }
            }
        }

        for j in 0..m {
            for i in 0..n {
                if board[j][i] == 'O' {
                    board[j][i] = 'X';
                } else if board[j][i] == 'Z' {
                    board[j][i] = 'O';
                }
            }
        }
    }
}
