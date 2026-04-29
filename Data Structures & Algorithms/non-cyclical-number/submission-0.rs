impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen = HashSet::new();
        let mut n = n;
        while n != 1 {
            if seen.contains(&n) {
                return false;
            }
            seen.insert(n);
            n = Solution::helper(n);
        }
        true
    }

    fn helper(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let d = n % 10;
            n /= 10;
            sum += d * d;
        }
        sum
    }
}
