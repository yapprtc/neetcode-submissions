impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut cost = vec![i32::MAX; points.len()];
        let mut set = HashSet::new();
        let mut min_heap = BinaryHeap::new();
        cost[0] = 0;
        min_heap.push(Reverse((0, 0)));
        while let Some(Reverse((c, u))) = min_heap.pop() {
            set.insert(u);
            for v in 0..points.len() {
                if set.contains(&v) {
                    continue;
                }
                let d = (points[v][0] - points[u][0]).abs() + (points[v][1] - points[u][1]).abs();
                if cost[v] > d {
                    cost[v] = d;
                    min_heap.push(Reverse((d, v)));
                }
            }
        }
        //println!("cost={:?}", cost);
        cost.into_iter().sum()
    }
}
