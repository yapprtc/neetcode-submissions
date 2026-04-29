impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        assert!(!nums.is_empty());
        let mut global = nums[0] as usize;
        let mut i = 1;
        while i < nums.len() && i <= global {
            global = std::cmp::max(i + (nums[i] as usize), global);
            i += 1;
        }
        global >= nums.len() - 1
    }
}
