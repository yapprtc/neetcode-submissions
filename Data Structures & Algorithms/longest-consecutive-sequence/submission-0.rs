impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut s:HashSet<i32> = HashSet::from_iter(nums.into_iter());
        let mut global = 0;

        while let Some(item) = s.iter().next().cloned() {
            let mut local = 1;
            let removed = s.take(&item).unwrap();
            // forward
            let mut prev = removed-1;
            while let Some(prev_item) = s.get(&prev).cloned() {
                local += 1;
                s.take(&prev_item);
                prev = prev_item-1;
            }
            // backward
            let mut next = removed+1;
            while let Some(next_item) = s.get(&next).cloned() {
                local += 1;
                s.take(&next_item);
                next = next_item+1;
            }
            global = std::cmp::max(global, local);
        }

        global
    }
}
