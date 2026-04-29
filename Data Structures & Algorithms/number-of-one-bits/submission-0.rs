impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut n = n;
        let mut c = 0;
        while n != 0 {
            n = n & (n - 1);
            c += 1;
        }
        c
    }
}
