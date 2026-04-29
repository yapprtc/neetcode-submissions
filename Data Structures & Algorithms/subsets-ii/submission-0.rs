impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let (mut result, mut a) = (vec![], vec![]);
        Solution::backtracking(&nums, &mut result, &mut a, 0);
        result
    }

    fn backtracking(nums:&[i32], result:&mut Vec<Vec<i32>>, a:&mut Vec<i32>, k: usize) {
        if k > nums.len() {
            return;
        } else if k == nums.len() {
            result.push(a.clone());
        } else {
            for candidate in [true, false] {
                let next = if candidate {
                    a.push(nums[k]);
                    k+1
                } else {
                    let mut next = k+1;
                    while next < nums.len() && nums[next-1] == nums[next] {
                        next += 1;
                    }
                    next
                };

                Solution::backtracking(nums, result, a, next);

                if candidate {
                    a.pop();
                }
            }
        }
    }
}
