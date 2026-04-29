impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s = s.as_bytes();
        let mut far = vec![0; 128];
        for i in 0..s.len() {
            far[s[i] as usize] = i;
        }
        let mut i = 0;
        let mut result = vec![];
        while i < s.len() {
            let mut j = far[s[i] as usize];
            let mut k = i+1;
            while k < j {
                j = std::cmp::max(j, far[s[k] as usize]);
                k += 1;
            }
            result.push((j-i+1) as i32);
            i = std::cmp::max(i+1, j+1);
        }
        result
    }
}
