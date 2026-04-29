impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut result, mut a) = (vec![], vec![]);
        Solution::backtracking(&nums, &mut result, &mut a, 0);
        result
    }

    fn backtracking(nums:&[i32], result: &mut Vec<Vec<i32>>, a:&mut Vec<i32>, mut k: usize) {
        if k == nums.len() {
            result.push(a.clone());
        } else {
            k += 1;
            for candidate in Solution::build_candidates(nums, a) {
                a.push(candidate);
                Solution::backtracking(nums, result, a, k);
                a.pop();
            }
        }
    }

    fn build_candidates(nums:&[i32], a:&[i32]) -> Vec<i32> {
        let mut in_perm = HashSet::new();
        for &num in a {
            in_perm.insert(num);
        }
        nums.iter().filter(|num| !in_perm.contains(num)).map(|&num| num).collect::<Vec<i32>>()
    }
}
