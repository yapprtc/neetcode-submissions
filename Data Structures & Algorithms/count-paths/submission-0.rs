impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut paths = vec![1i32; n as usize];
        for j in 1..m as usize {
            for i in 1..n as usize {
                paths[i] += paths[i-1];
            }
        }
        paths[n as usize - 1]
    }
}
