impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in tokens {
            if token == "+" || token == "-" || token == "*" ||token == "/" {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let result = if token == "+" {
                    left + right
                } else if token == "-" {
                    left - right
                } else if token == "*" {
                    left * right
                } else {
                    left / right
                };
                stack.push(result);
            } else {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
        assert!(!stack.is_empty());
        stack.pop().unwrap()
    }
}
