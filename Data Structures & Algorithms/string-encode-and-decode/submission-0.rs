impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        // vec_len : { str_len : str, ...}
        let mut result = String::new();
        result += &format!("{}:{{", strs.len());
        for s in strs {
            result += &format!("[{}:{}]", s.len(), s);
        }
        result += "}";
        result
    }

    pub fn decode(s: String) -> Vec<String> {
        let b = s.as_bytes();
        let mut result = vec![];
        let (mut i, mut j) = (0usize, 0usize);
        while j < b.len() && b[j] != b':' {
            j += 1;
        }
        let vlen = s.get(i..j).unwrap().parse().unwrap();
        j += 2; // skip ":{"
        for k in 0..vlen {
            j += 1; // skip "["
            i = j;
            while j < b.len() && b[j] != b':' {
                j += 1;
            }
            let slen: usize = s.get(i..j).unwrap().parse().unwrap();
            j += 1; // skip ":"
            result.push(s.get(j..j+slen).unwrap().to_string());
            j += slen;
            j += 1; // skip "]"
        }
        result
    }
}
