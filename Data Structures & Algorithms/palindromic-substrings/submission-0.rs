impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len() as i32;
        
        let mut count = 0;
        for i in 0..n {
            count += 1;
            let mut offset = 1;
            while i - offset >= 0 && i + offset < n && 
                s[(i - offset) as usize] == s[(i + offset) as usize] {
                count += 1;
                offset += 1;        
            }

            let mut left = i;
            let mut right = i + 1;
            while left >= 0 && right < n && s[left as usize] == s[right as usize] {
                count += 1;
                left -= 1;
                right += 1;        
            }
        }
        count
    }
}
