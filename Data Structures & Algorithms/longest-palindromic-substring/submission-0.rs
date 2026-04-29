impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s
        }
        let b = s.as_bytes();
        let (lo, hi) = (0i32, b.len() as i32 - 1);
        
        let (mut global, mut start) = (0, 0);
        for i in 0..b.len() as i32 {
            let odd = Solution::palindrome_odd(b, i, lo, hi);
            if odd > global {
                global = odd;
                start = i - odd/2;
            }
            let even = Solution::palindrome_even(b, i, lo, hi);
            if even > global {
                global = even;
                start = i - (even - 1)/2;
            }
        }

        s[start as usize..(start+global) as usize].to_string()
    }

    fn palindrome_odd(s:&[u8], center: i32, lo: i32, hi: i32) -> i32 {
        let (mut offset, mut local) = (1, 1);
        while center - offset >= lo && center + offset <= hi && 
              s[(center - offset) as usize] == s[(center + offset) as usize] {
            local += 2;
            offset += 1;        
        }
        local
    }

    fn palindrome_even(s:&[u8], mut left: i32, lo: i32, hi: i32) -> i32 {
        let (mut right, mut local) = (left+1, 0);
        while left >= lo && right <= hi && s[left as usize] == s[right as usize] {
            local += 2;
            left -= 1;
            right += 1;        
        }
        local
    }
}
