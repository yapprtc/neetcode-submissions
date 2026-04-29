impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        let (mut local, mut global) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            local = std::cmp::max(local + nums[i], nums[i]);
            global = std::cmp::max(local, global);
        }
        global
    }
}
