impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        assert!(num_courses>=1);
        let (mut onStack, mut visited) = (vec![false; num_courses as usize], vec![false; num_courses as usize]);
        let (mut graph, mut postVisited) = (vec![], vec![]);
        for _ in 0..num_courses as usize {
            graph.push(vec![]);
        }

        for prerequisity in prerequisites {
            graph[prerequisity[1] as usize].push(prerequisity[0]);
        }

        for i in 0..num_courses {
            if !visited[i as usize] && Solution::dfs(&graph, &mut onStack, &mut visited, i, &mut postVisited) {
                return vec![];
            }
        }
        postVisited.reverse();
        postVisited
    }

    fn dfs(graph:&Vec<Vec<i32>>, onStack:&mut [bool], visited: &mut [bool], curr: i32, postVisited:&mut Vec<i32>) -> bool {
        //preVisited:
        onStack[curr as usize] = true;
        visited[curr as usize] = true;

        for &next in &graph[curr as usize] {
            if onStack[next as usize] {
                return true;
            } else if !visited[next as usize] && Solution::dfs(graph, onStack, visited, next, postVisited) {
                return true;
            }
        }

        onStack[curr as usize] = false;
        postVisited.push(curr);

        false
    }
}
