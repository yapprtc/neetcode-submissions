impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digit2letter:Vec<Vec<char>> = vec![
            vec![], //0
            vec![], //1
            vec!['a', 'b', 'c'], //2
            vec!['d', 'e', 'f'], //3
            vec!['g', 'h', 'i'], //4
            vec!['j', 'k', 'l'], //5
            vec!['m', 'n', 'o'], //6
            vec!['p', 'q', 'r', 's'], //7
            vec!['t', 'u', 'v'], //8
            vec!['w', 'x', 'y', 'z'], //9
        ];
        let (mut result, mut a) = (vec![], String::new());
        Solution::backtracking(&digit2letter, digits.as_bytes(), &mut result, &mut a, 0);
        result
    }

    fn backtracking(digit2letter: &Vec<Vec<char>>, digits:&[u8], result:&mut Vec<String>, a: &mut String, mut k: usize) {
        if k == digits.len() {
            if !a.is_empty() {
                result.push(a.clone());
            }
        } else {
            let d = (digits[k] - b'0') as usize;
            k += 1;
            for &c in &digit2letter[d] {
                a.push(c);
                Solution::backtracking(digit2letter, digits, result, a, k);
                a.pop();
            }
        }
    }
}
