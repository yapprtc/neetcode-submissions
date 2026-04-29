impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let (num1, num2) = if num1.len() < num2.len() {
            (num2, num1)
        } else {
            (num1, num2)
        };

        let mut res = String::new();
        let num2_bytes = num2.as_bytes();
        for (z, &d) in num2_bytes.iter().rev().enumerate() {
            let cur = Self::mul_digit(&num1, d, z);
            res = Self::add_strings(&res, &cur);
        }

        res
    }

    fn mul_digit(s: &str, d: u8, zero: usize) -> String {
        let s_bytes = s.as_bytes();
        let mut i = s_bytes.len() as i32 - 1;
        let mut carry = 0u32;
        let digit = (d - b'0') as u32;
        let mut cur = Vec::new();

        while i >= 0 || carry > 0 {
            let n = if i >= 0 { (s_bytes[i as usize] - b'0') as u32 } else { 0 };
            let prod = n * digit + carry;
            cur.push((b'0' + (prod % 10) as u8) as char);
            carry = prod / 10;
            i -= 1;
        }

        cur.reverse();
        let mut result: String = cur.into_iter().collect();
        result.extend(std::iter::repeat('0').take(zero));
        result
    }

    fn add_strings(num1: &str, num2: &str) -> String {
        let a = num1.as_bytes();
        let b = num2.as_bytes();
        let mut i = a.len() as i32 - 1;
        let mut j = b.len() as i32 - 1;
        let mut carry = 0u32;
        let mut res = Vec::new();

        while i >= 0 || j >= 0 || carry > 0 {
            let n1 = if i >= 0 { (a[i as usize] - b'0') as u32 } else { 0 };
            let n2 = if j >= 0 { (b[j as usize] - b'0') as u32 } else { 0 };
            let total = n1 + n2 + carry;
            res.push((b'0' + (total % 10) as u8) as char);
            carry = total / 10;
            i -= 1;
            j -= 1;
        }

        res.reverse();
        res.into_iter().collect()
    }
}
