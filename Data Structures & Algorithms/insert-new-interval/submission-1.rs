impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();

        let n = intervals.len();
        if n == 0 {
            return vec![new_interval];
        } else if new_interval[1] < intervals[0][0] {
            intervals.insert(0, new_interval);
            return intervals;
        } else if new_interval[0] > intervals[n-1][1] {
            intervals.push(new_interval);
            return intervals;
        }
        
        let mut new_interval = new_interval;
        let mut result = vec![];
        for interval in intervals {
            //println!("interval = {:?}", interval);
            if interval[1] < new_interval[0] {
                //println!("result.push(interval) = {:?}", interval);
                result.push(interval);
            } else if interval[0] >= new_interval[0] && interval[0] <= new_interval[1] {
                new_interval[1] = std::cmp::max(interval[1], new_interval[1]);
            } else if interval[1] >= new_interval[0] && interval[1] <= new_interval[1] {
                new_interval[0] = std::cmp::min(interval[0], new_interval[0]);
            } else {
                //println!("result.push(new_interval) = {:?}", new_interval);
                if !(new_interval[0] >= interval[0] && new_interval[1] <= interval[1]) {
                    result.push(new_interval);        
                }
                
                new_interval = interval;
            }
            //println!("new_interval = {:?}", new_interval);
        }
        result.push(new_interval);

        result
    }
}
