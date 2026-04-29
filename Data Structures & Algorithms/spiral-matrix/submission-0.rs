impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len() as i32;
        let n = matrix[0].len() as i32;
        let mut k = m * n;
        let mut result = vec![];
        let mut offset = 0 as i32;
        while k > 0 {
            for i in offset..n-offset {
                result.push(matrix[offset as usize][i as usize]);
                k-=1;
                if (k == 0){
                    return result;
                }
            }
            for j in offset+1..m-offset {
                result.push(matrix[j as usize][(n-1-offset) as usize]);
                k-=1;
                if (k == 0){
                    return result;
                }
            }
            for i in (offset..n-offset-1).rev() {
                result.push(matrix[(m-offset-1) as usize][i as usize]);
                k-=1;
                if (k == 0){
                    return result;
                }
            }
            for j in (offset+1..m-offset-1).rev() {
                result.push(matrix[j as usize][offset as usize]);
                k-=1;
                if (k == 0){
                    return result;
                }
            }

            offset += 1;
        }
        result
    }
}
