impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let s = s.as_bytes();
        let (mut i, mut j) = (0 as i32, s.len() as i32 - 1);
        while i < j {
            while i < j && !s[i as usize].is_ascii_alphanumeric() {
                i += 1;
            }
            while i < j && !s[j as usize].is_ascii_alphanumeric() {
                j -= 1;
            }
            if !s[i as usize].eq_ignore_ascii_case(&s[j as usize]) {
                return false
            }
            i += 1;
            j -= 1;
        }

        true
    }
}
