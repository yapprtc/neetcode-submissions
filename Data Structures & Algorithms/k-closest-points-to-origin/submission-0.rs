use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        assert!(k >= 1);
        let mut max_heap = BinaryHeap::new();
        for point in points {
            let distance = point[0]*point[0] + point[1]*point[1];
            if max_heap.len() < k as usize {
                max_heap.push((distance, point[0], point[1]));
            } else {
                let top = max_heap.peek().unwrap();
                if top.0 > distance {
                    max_heap.pop();
                    max_heap.push((distance, point[0], point[1]));
                }
            }
        }
        let mut result = vec![];
        while let Some(top) = max_heap.pop() {
            result.push(vec![top.1, top.2]);
        }
        result
    }
}
