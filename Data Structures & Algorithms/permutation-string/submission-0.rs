impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (mut matches, mut mismatches) = (0, 0);
        let mut map = [0; 128];
        for &ch in s1.as_bytes() {
            if map[ch as usize] == 0 {
                matches += 1;
            }
            map[ch as usize] += 1;
        }

        let s2 = s2.as_bytes();
        let (mut begin, mut end) = (0, 0);
        while end < s2.len() {
            if map[s2[end] as usize] == 1 {
                matches -= 1;
                if matches == 0 {
                    return true;
                }
            } else if map[s2[end] as usize] == 0 {
                mismatches += 1;
            }
            map[s2[end] as usize] -= 1;
            end += 1;

            while mismatches > 0 {
                if map[s2[begin] as usize] == 0 {
                    matches += 1;
                }
                map[s2[begin] as usize] += 1;
                if map[s2[begin] as usize] == 0 {
                    mismatches -= 1;
                }
                begin += 1;
            }
        }
        false
    }
}
