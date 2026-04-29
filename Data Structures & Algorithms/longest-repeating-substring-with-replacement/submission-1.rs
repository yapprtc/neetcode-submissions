impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut map = [0i32; 128];
        let (mut global, mut count) = (0, 0);
        let (mut begin, mut end) = (0, 0);
        while end < s.len() {
            map[s[end] as usize] += 1;
            count = std::cmp::max(count, map[s[end] as usize]);
            end += 1;
            while (end - begin) as i32 - count > k {
                map[s[begin] as usize] -= 1;
                begin += 1;
            }
            global = std::cmp::max(global, end - begin);
        }
        global as i32
    }
}
