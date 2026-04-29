impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set = HashSet::new();
        // row
        for j in 0..9 {
            set.clear();
            for i in 0..9 {
                if board[j][i] != '.' {
                    if set.contains(&board[j][i]) {
                        return false;
                    }
                    set.insert(board[j][i]);
                }
            }
        }

        // col
        for i in 0..9 {
            set.clear();
            for j in 0..9 {
                if board[j][i] != '.' {
                    if set.contains(&board[j][i]) {
                        return false;
                    }
                    set.insert(board[j][i]);
                }
            }
        }

        // 3x3
        for j in (0..9).step_by(3) {
            for i in (0..9).step_by(3) {
                set.clear();
                for m in 0..3 {
                    for n in 0..3 {
                        if board[j+m][i+n] != '.' {
                            if set.contains(&board[j+m][i+n]) {
                                return false;
                            }
                            set.insert(board[j+m][i+n]);
                        }
                    }
                }
            }
        }


        true
    }
}
