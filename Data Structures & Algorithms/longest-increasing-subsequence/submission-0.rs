impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        let mut l = vec![0; nums.len()];
        let mut global = 1;
        l[0] = 1;
        for j in 1..nums.len() {
            let mut local = 0;
            for i in 0..j {
                if nums[i] < nums[j] {
                    local = std::cmp::max(local, l[i]);
                }
            }
            l[j] = 1 + local;
            global = std::cmp::max(global, l[j]);
        }
        
        global
    }
}
