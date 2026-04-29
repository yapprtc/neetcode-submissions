impl Solution {
    pub fn is_valid(s: String) -> bool {
        let map = HashMap::from([
            (b')', b'('),
            (b'}', b'{'),
            (b']', b'['),
        ]);

        let mut stack = vec![];
        for ch in s.as_bytes() {
            if let Some(&l) = map.get(&ch) {
                if stack.pop() != Some(l) {
                    return false; 
                }
            } else {
                stack.push(*ch);
            }
        }

        stack.is_empty()
    }
}
