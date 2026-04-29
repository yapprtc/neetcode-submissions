impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len(); assert!(m!=0);
        let n = matrix[0].len(); assert!(n!=0);

        // first search column
        let mut array = vec![];
        for j in 0..m {
            array.push(matrix[j][0]);
        }
        let j = Solution::binary_search(&array, target);
        if j < 0 {
            return false;
        } else if matrix[j as usize][0] == target {
            return true;
        }

        // then search row
        let i = Solution::binary_search(&matrix[j as usize], target);
        matrix[j as usize][i as usize] == target
    }

    fn binary_search(array: &[i32], target: i32) -> i32 {
        let (mut lo, mut hi) = (0i32, array.len() as i32 - 1);
        while lo <= hi {
            let mi = lo + (hi - lo) / 2;
            if array[mi as usize] == target {
                return mi;
            } else if array[mi as usize] < target {
                lo = mi + 1;
            } else {
                hi = mi - 1;
            }
        }
        return hi;
    }
}
