impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        let mut money = vec![0i32; nums.len()];
        money[0] = nums[0];
        let mut global = nums[0];
        for j in 1..nums.len() {
            let mut local = 0;
            for i in 0..j-1 {
                local = std::cmp::max(local, money[i]);
            }
            money[j] = nums[j] + local;
            global = std::cmp::max(global, money[j]);
        }
        global
    }
}
