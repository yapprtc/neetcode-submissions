impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        assert!(!numbers.is_empty());
        let (mut lo, mut hi) = (0, numbers.len() - 1);
        while lo < hi {
            let sum = numbers[lo] + numbers[hi];
            if sum == target {
                return vec![lo as i32 + 1, hi as i32 + 1];
            } else if sum > target {
                hi -= 1;
            } else {
                lo += 1;
            }
        }

        vec![-1, -1]
    }
}
