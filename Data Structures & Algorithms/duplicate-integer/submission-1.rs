impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut s = HashSet::new();
        for num in nums {
            if !s.insert(num) {
                return true
            }
        }
        false
    }
}
