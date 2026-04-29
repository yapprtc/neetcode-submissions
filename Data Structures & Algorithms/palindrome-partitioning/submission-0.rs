impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let (mut result, mut a) = (vec![], vec![]);
        Solution::backtracking(&s, &mut result, &mut a);
        result
    }

    fn backtracking(s:&str, result: &mut Vec<Vec<String>>, a: &mut Vec<String>) {
        if s.is_empty() {
            result.push(a.clone());
        } else {
            for k in 1..=s.len() {
                if Solution::is_palindrome(&s[..k]) {
                    a.push(s[..k].to_string());

                    Solution::backtracking(&s[k..], result, a);

                    a.pop();
                }
            }
        }
    }

    fn is_palindrome(a:&str) -> bool {
        if a.is_empty() {
            return true;
        }
        let a = a.as_bytes();
        let (mut lo, mut hi) = (0i32, a.len() as i32 - 1);
        while lo <= hi && a[lo as usize] == a[hi as usize]{
            lo += 1;
            hi -= 1;
        }
        lo > hi
    }
}
