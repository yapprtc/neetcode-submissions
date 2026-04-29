impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut times = vec![];
        for (&position, &speed) in position.iter().zip(speed.iter()) {
            times.push((position, (target - position) as f32 / (speed as f32)));
        }
        times.sort_by(|a, b| b.0.cmp(&a.0));
        //println!("times={:?}", times);
        let mut stack = vec![];
        for time in times {
            if stack.is_empty() || stack[stack.len() - 1] < time.1 {
                stack.push(time.1);
            }
        }
        stack.len() as i32
    }
}
