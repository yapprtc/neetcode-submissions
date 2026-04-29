impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len(); assert!(n>=3);
        let (mut pi, mut rank) = (vec![0i32; n+1], vec![0i32; n+1]);
        for i in 0..=n {
            pi[i] = i as i32;
        }

        for edge in edges {
            let p0 = Solution::find(edge[0], &mut pi);
            let p1 = Solution::find(edge[1], &mut pi);
            if p0 == p1 {
                return edge;
            }
            Solution::union(edge[0], edge[1], &mut pi, &mut rank);
        }
        vec![]
    }

    fn find(x: i32, pi:&mut [i32]) -> i32 {
        if x != pi[x as usize] {
            pi[x as usize] = Solution::find(pi[x as usize], pi);
        }
        return pi[x as usize];
    }

    fn union(x: i32, y: i32, pi:&mut [i32], rank:&mut [i32]) {
        let px = Solution::find(pi[x as usize], pi);
        let py = Solution::find(pi[y as usize], pi);
        if rank[px as usize] < rank[py as usize] {
            pi[px as usize] = py;
        } else {
            pi[py as usize] = px;
            if rank[px as usize] == rank[py as usize] {
                rank[px as usize] += 1;
            }
        }
    }
}
