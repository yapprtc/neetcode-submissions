impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        // min_local = min(min_local * nums[j], nums[j])
        // max_local = max(max_local * nums[j], min_local * nums[j], nums[j])
        // global = max(global, max_local)
        assert!(!nums.is_empty());
        let (mut min_local, mut max_local, mut global) = (nums[0], nums[0], nums[0]);
        for j in 1..nums.len() {
            let prev_max_local = max_local;
            max_local = std::cmp::max(std::cmp::max(max_local * nums[j], min_local * nums[j]), nums[j]);
            min_local = std::cmp::min(std::cmp::min(min_local * nums[j], prev_max_local*nums[j]), nums[j]);
            global = std::cmp::max(global, max_local);
        }
        global
    }
}
