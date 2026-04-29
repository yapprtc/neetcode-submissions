impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = HashMap::new();
        for num in nums {
            if let Some(count) = freq.get_mut(&num) {
                *count += 1;
            } else {
                freq.insert(num, 1);
            }
        }
        let mut heap = BinaryHeap::new();
        for (num, count) in freq {
            heap.push(Reverse((count, num)));
            if heap.len() as i32 > k {
                heap.pop();
            }
        }
        heap.into_iter().map(|p| p.0.1).collect()
    }
}
