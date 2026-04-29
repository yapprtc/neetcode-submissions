impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        assert!(!intervals.is_empty());

        let mut intervals = intervals;
        intervals.sort();

        let mut result = vec![];
        let mut prev_interval = vec![intervals[0][0],intervals[0][1]];
        for i in 1..intervals.len() {
            if intervals[i][0] <= prev_interval[1] {
                prev_interval[1] = std::cmp::max(intervals[i][1], prev_interval[1]);
            } else {
                result.push(prev_interval);
                prev_interval = vec![intervals[i][0], intervals[i][1]];
            }
        }
        result.push(prev_interval);

        result
    }
}
