impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 0i32;
        Solution::backtracking(&nums, &mut result, 0, 0, target);
        result
    }

    fn backtracking(nums:&[i32], result:&mut i32, mut k: usize, mut sum: i32, target: i32) {
        if k == nums.len() {
            if sum == target {
                *result += 1;
            }
        } else {
            k += 1;
            for op in [1, -1] {
                sum += op * nums[k - 1];
                Solution::backtracking(nums, result, k, sum, target);
                sum -= op * nums[k - 1];
            }
        }
    }
}
