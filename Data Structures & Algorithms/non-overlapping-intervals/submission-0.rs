impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        assert!(!intervals.is_empty());
        let mut intervals = intervals;
        intervals.sort();

        let mut count = 0;
        let mut prev_end = intervals[0][1];
        for i in 1..intervals.len() {
            if intervals[i][0] < prev_end {
                prev_end = std::cmp::min(prev_end, intervals[i][1]);
                count += 1;
            } else {
                prev_end = intervals[i][1];
            }
        }
        count
    }
}
