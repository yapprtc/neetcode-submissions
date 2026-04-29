impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0f64 || n == 1i32 {
            return x;
        }
        if n == 0 {
            return 1f64;
        } 
        if n < 0 {
            return 1.0 / Solution::my_pow(x, -n);
        }

        let y = Solution::my_pow(x, n/2);
        if n % 2 == 1 {
            return x * y * y;
        } else {
            return y * y;
        }
    }
}
