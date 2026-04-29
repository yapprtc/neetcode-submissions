impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack:Vec<(i32, usize)> = vec![];
        let mut result = vec![0i32; temperatures.len()];
        for i in 0..temperatures.len() {
            if stack.is_empty() || temperatures[i] <= stack[stack.len() - 1].0 {
                stack.push((temperatures[i], i));
            } else {
                while !stack.is_empty() && stack[stack.len() - 1].0 < temperatures[i] {
                    let (_, j) = stack.pop().unwrap();
                    result[j] = (i - j) as i32;
                }
                stack.push((temperatures[i], i));
            }
            //println!("i = {}, stack={:?}, result = {:?}", i, stack, result);
        }
        
        result
    }
}
