impl Solution {
    pub fn reverse_bits(n: u32) -> u32 {
        let mut res = 0;
        for i in 0..32 {
            if ((n>>i)&1) == 1 {
                res |= 1<<(31-i);
            }
        }
        res
    }
}
