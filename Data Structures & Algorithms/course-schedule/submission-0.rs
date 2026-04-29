impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: Vec<Vec<usize>> = vec![];
        for _ in 0..num_courses {
            graph.push(vec![]);
        }
        for prerequisity in prerequisites {
            graph[prerequisity[1] as usize].push(prerequisity[0] as usize);
        }

        for i in 0..num_courses as usize {
            let mut onstack = vec![false; num_courses as usize];
            if !Solution::dfs(&graph, &mut onstack, i) {
                return false;
            }
        }
        true
    }

    fn dfs(graph: &Vec<Vec<usize>>, onstack: &mut [bool], i: usize) -> bool {
        onstack[i] = true;
        for j in &graph[i] {
            if onstack[*j] || !Solution::dfs(graph, onstack, *j) {
                return false;
            }
        }
        onstack[i] = false;
        true
    }
}
