impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // E[i,j] = min(1+E[i, j-1], 1+E[i-1,j], diff(i,j)+E[i-1, j-1])
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (word1.len(), word2.len());
        let mut E = vec![];
        for _ in 0..=m {
            E.push(vec![0i32; n + 1]);
        }
        for j in 1..=m {
            E[j][0] = j as i32;
        }
        for i in 1..=n {
            E[0][i] = i as i32;
        }
        for j in 1..=m {
            for i in 1..=n {
                let diff = if word1[j-1] == word2[i-1] {
                    0
                } else {
                    1
                };
                E[j][i] = std::cmp::min(std::cmp::min(1+E[j-1][i], 1+E[j][i-1]), diff+E[j-1][i-1]);
            }
        }
        E[m][n]
    }
}
