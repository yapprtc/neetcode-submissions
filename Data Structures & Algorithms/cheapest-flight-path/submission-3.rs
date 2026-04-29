impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (mut graph, mut dist) = (vec![], vec![]);
        for _ in 0..n as usize {
            graph.push(vec![]);
            dist.push(vec![i32::MAX; k as usize + 2]);
        }
        for flight in flights {
            graph[flight[0] as usize].push((flight[1] as usize, flight[2]));
        }
        dist[src as usize][0] = 0;
        // dist[v][i] = min(dist[u][i-1]+ l[u][v], if u,v has edge)
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, src as usize, 0usize)));
        while let Some(Reverse((cost, u, i))) = queue.pop() {
            //println!("u={}, i={}", u, i);
            for &(v, price) in &graph[u] {
                //println!("v={}, price={}", v, price);
                if i <= k as usize && dist[u][i]+price < dist[v][i+1]{
                    //println!("dist[v][i+1]={}, dist[u][i]={}, price={}", dist[v][i+1], dist[u][i], price);
                    dist[v][i+1] = dist[u][i]+price;
                    queue.push(Reverse((dist[v][i+1], v, i+1)));
                }
            }
        }
        println!("dist={:?}", dist);
        let mut result = i32::MAX;
        for i in 0..k as usize + 2 {
            result = std::cmp::min(result, dist[dst as usize][i]);
        }
        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}
