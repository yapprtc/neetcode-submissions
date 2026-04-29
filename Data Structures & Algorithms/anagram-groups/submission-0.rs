impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m: HashMap<[u16; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let freq = Self::str2freq(s.as_bytes());
            if let Some(v) = m.get_mut(&freq) {
                v.push(s);
            } else {
                m.insert(freq, vec![s]);
            }
        }
        m.into_values().collect()
    }

    fn str2freq(s: &[u8]) -> [u16; 26] {
        let mut freq = [0u16; 26];
        for ch in s {
            freq[(ch - b'a') as usize] += 1;
        }
        freq
    }
}
