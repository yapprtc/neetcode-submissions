impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let (mut local, mut global) = (prices[0], 0);
        for i in 1..prices.len() {            
            global = std::cmp::max(prices[i]-local, global);
            local = std::cmp::min(local, prices[i]);
        }  
        global  
    }
}
