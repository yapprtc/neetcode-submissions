impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut a = vec![];
        Solution::backtracking(&nums, &mut result, &mut a, 0);
        result
    }

    fn backtracking(nums:&[i32], result: &mut Vec<Vec<i32>>, a: &mut Vec<i32>, mut k: usize) {
        if k == nums.len() {
            result.push(a.clone());
        } else {
            k += 1;
            for candidate in [true, false] {
                if candidate {
                    a.push(nums[k-1]);
                }
                Solution::backtracking(nums, result, a, k);
                if candidate {
                    a.pop();
                }
            }
        }
    }
}
