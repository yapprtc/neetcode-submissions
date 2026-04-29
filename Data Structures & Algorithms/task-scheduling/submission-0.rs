impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq = HashMap::new();
        for task in tasks {
            freq.entry(task)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let vec: Vec<(i32, char)> = freq.into_iter().map(|(key, val)| (val, key)).collect();
        let mut max_heap = BinaryHeap::from(vec);

        let mut result = 0;
        while !max_heap.is_empty() {
            let mut temp = BinaryHeap::new();
            for i in 0..=n {
                if let Some((count, key)) = max_heap.pop() {
                    if count > 1 {
                        temp.push((count-1, key));
                    }
                    result += 1;
                } else if !temp.is_empty() {
                    result += 1;
                }
            }

            if !temp.is_empty() {
                max_heap.append(&mut temp);
            }
        }
        result
    }
}
