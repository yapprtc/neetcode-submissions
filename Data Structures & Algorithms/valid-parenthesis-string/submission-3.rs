impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut left_stack, mut star_stack) = (vec![], vec![]);
        for (i, &c) in s.as_bytes().iter().enumerate() {
            if c == b'(' {
                left_stack.push(i);
            } else if c == b'*' {
                star_stack.push(i);
            } else if c == b')' {
                if !left_stack.is_empty() {
                    left_stack.pop();
                } else if !star_stack.is_empty() {
                    star_stack.pop();
                } else {
                    return false;
                }
            }
        }

        while let Some(l) = left_stack.pop() {
            if let Some(t) = star_stack.pop() {
                if t < l {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}
