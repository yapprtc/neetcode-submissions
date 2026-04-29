impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        let mut money = vec![];
        money.push(vec![0i32; nums.len() - 1]);
        money.push(vec![0i32; nums.len() - 1]);
        money[0][0] = nums[0];
        money[1][0] = nums[1];
        
        let mut global = std::cmp::max(nums[0], nums[1]);
        for start in [0, 1] {
            for j in 1..nums.len() - 1 {
                let mut local = 0;
                for i in 0..j-1 {
                    local = std::cmp::max(local, money[start as usize][i]);
                }
                money[start as usize][j] = nums[j + start] + local;
                global = std::cmp::max(global, money[start as usize][j]);
            }
        }
        global
    }
}
