impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        assert!(n>=1 && k>=1);
        let mut dg: Vec<Vec<(usize, i32)>> = vec![];
        for _ in 0..n as usize{
            dg.push(vec![]);
        }
        for time in times {
            dg[time[0] as usize - 1].push((time[1] as usize - 1, time[2]));
        }

        let mut min_heap = BinaryHeap::new();
        let mut dist = vec![i32::MAX; n as usize];
        let u = (k - 1) as usize;
        dist[u] = 0;
        min_heap.push(Reverse((dist[u], u)));

        while let Some(Reverse((d, u))) = min_heap.pop() {
            for &(v, time) in &dg[u] {
                if dist[v] > dist[u] + time {
                    dist[v] = dist[u] + time;
                    min_heap.push(Reverse((dist[v], v)));
                }
            }
        }

        let mut time = 0;
        for d in dist {
            if d == i32::MAX {
                return -1;
            }
            time = std::cmp::max(time, d);
        }
        time
    }
}
