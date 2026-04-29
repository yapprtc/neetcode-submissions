impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.iter().sum::<i32>() < cost.iter().sum::<i32>() {
            return -1;
        }
        let (mut tank, mut i) = (gas[0], 0);
        let mut result = 0;
        while i+1 < gas.len() {
            i += 1;
            let remain = tank - cost[i - 1];
            if remain < 0 {
                tank = gas[i];
                result = i;
            } else {
                tank = gas[i] + remain;
            }
        }
        result as i32
    }
}
