impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let s = s.as_bytes();
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            if !s[i].is_ascii_alphanumeric() {
                i += 1;
                continue;
            }
            if !s[j].is_ascii_alphanumeric() {
                j -= 1;
                continue;
            }
            if !s[i].eq_ignore_ascii_case(&s[j]) {
                return false
            }
            i += 1;
            j -= 1;
        }

        true
    }
}
