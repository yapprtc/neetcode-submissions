impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![];
        result.reserve((n+1) as usize);
        for i in 0..=n {
            result.push(Solution::count(i));
        }
        result
    }

    fn count(mut n: i32) -> i32 {
        let mut c = 0;
        while n != 0 {
            n = n & (n - 1);
            c += 1;
        }
        c
    }
}
