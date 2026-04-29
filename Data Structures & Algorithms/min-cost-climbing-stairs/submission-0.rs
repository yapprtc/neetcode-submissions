impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        if cost.len() < 2 {
            return 0;
        }
        let (mut fn_1, mut fn_2) = (cost[1], cost[0]);
        let mut fn_0 = 0;
        for n in 2..cost.len() {
            fn_0 = std::cmp::min(cost[n]+fn_1, cost[n]+fn_2);
            fn_2 = fn_1;
            fn_1 = fn_0;
        }
        std::cmp::min(fn_1, fn_2)
    }
}
