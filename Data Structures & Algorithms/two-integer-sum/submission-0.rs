impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut diffs = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(j) = diffs.get(num) {
                return vec![*j, i as i32];
            } else {
                diffs.insert(target - *num, i as i32);
            }
        }
        vec![-1, -1]
    }
}
