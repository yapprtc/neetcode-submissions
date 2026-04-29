impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![];
        for _ in 0..n as usize {
            graph.push(vec![]);
        }
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);    
        }

        let mut visited = vec![false; n as usize];
        let mut cc = 0;
        for i in 0..n as usize {
            if !visited[i] {
                if !Solution::dfs(&graph, &mut visited, n as usize, i) {
                    return false;
                }
                cc += 1;
            }
        }
        cc == 1
    }

    fn dfs(graph:&Vec<Vec<usize>>, visited: &mut [bool], parent: usize, i: usize) -> bool {
        visited[i] = true;
        for &next in &graph[i] {
            if next != parent && visited[next] {
                return false;
            } if !visited[next] {
                if !Solution::dfs(graph, visited, i, next) {
                    return false;
                }
            }
        }

        true
    }
}
