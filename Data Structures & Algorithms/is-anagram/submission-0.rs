impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut hs = vec![0; 27];
        let mut ht = vec![0; 27];
        for ch in s.as_bytes() {
            hs[(ch-b'a') as usize] += 1;
        }
        for ch in t.as_bytes() {
            ht[(ch-b'a') as usize] += 1;
        }
        for i in 0..27 {
            if hs[i] != ht[i] {
                return false;
            }
        }
        true
    }
}
