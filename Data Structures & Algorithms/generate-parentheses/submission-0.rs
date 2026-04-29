impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let (mut result, mut a) = (vec![], String::new());
        Solution::backtracking(n, 0, 0, &mut result, &mut a);
        result
    }

    fn backtracking(n: i32, mut l: i32, mut r: i32, result:&mut Vec<String>, a:&mut String) {
        if l > n {
            return;
        } else if l == n {
            if r == n {
                result.push(a.clone());
            }
        } else {
            l += 1;
            a.push('(');
            for i in 0..=l-r {
                for j in 0..i {
                    a.push(')');
                } 

                Solution::backtracking(n, l, r+i, result, a);

                for j in 0..i {
                    a.pop();
                } 
            }
            a.pop();
        }
    }
}
