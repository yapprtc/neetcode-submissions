impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (mut c, mut r) = (false, false);
        let (n, m) = (matrix.len(), matrix[0].len());
        for j in 0..n {
            if matrix[j][0] == 0 {
                r = true;
                break;
            }
        }
        for i in 0..m {
            if matrix[0][i] == 0 {
                c = true;
                break;
            }
        }
        for j in 1..n {
            for i in 1..m {
                if matrix[j][i] == 0 {
                    matrix[j][0] = 0;
                    matrix[0][i] = 0;
                }
            }
        }
        for j in 1..n {
            if matrix[j][0] == 0 {
                for i in 1..m {
                    matrix[j][i] = 0;
                }
            }
        }
        for i in 1..m {
            if matrix[0][i] == 0 {
                for j in 1..n {
                    matrix[j][i] = 0;
                }
            }
        }
        if r {
            for j in 0..n {
                matrix[j][0] = 0;
            }
        }
        if c {
            for i in 0..m {
                matrix[0][i] = 0;
            }
        }
    }
}
