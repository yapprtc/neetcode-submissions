impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        // first row swap
        for j in 0..n/2 {
            matrix.swap(j, n-1-j);
        }
        // then diagal swap
        for j in 0..n {
            for i in j+1..n {
                let tmp = matrix[j][i];
                matrix[j][i] = matrix[i][j];
                matrix[i][j] = tmp;
            }
        }
    }
}
