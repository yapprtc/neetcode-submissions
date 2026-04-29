/**
 * Definition of Interval:
 * #[derive(Debug, Clone)]
 * pub struct Interval {
 *     pub start: i32,
 *     pub end: i32,
 * }
 *
 * impl Interval {
 *     pub fn new(start: i32, end: i32) -> Self {
 *         Interval { start, end }
 *     }
 * }
 */

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
        let mut map = BTreeMap::new();
        for interval in intervals {
            map.entry(interval.start).and_modify(|curr| *curr += 1).or_insert(1);
            map.entry(interval.end).and_modify(|curr| *curr -= 1).or_insert(-1);
        }
        let (mut global, mut local) = (0i32, 0i32);
        for count in map.into_values() {
            local += count;
            global = std::cmp::max(global, local);
        }
        global
    }
}
