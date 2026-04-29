impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = candidates;
        nums.sort();

        let (mut result, mut a) = (vec![], vec![]);
        Solution::backtracking(&nums, &mut result, &mut a, 0, 0, target);
        result
    }

    fn backtracking(nums:&[i32], result:&mut Vec<Vec<i32>>, a:&mut Vec<i32>, mut k: usize, mut sum: i32, target: i32) {
        if sum > target || k > nums.len() {
            return;
        } else if k == nums.len() {
            if sum == target {
                result.push(a.clone());
            }
        } else {
            for candidate in [true, false] {
                let next = if candidate {
                    a.push(nums[k]);
                    sum += nums[k];
                    k+1
                } else {
                    let mut next = k+1;
                    while next < nums.len() && nums[next-1] == nums[next]{
                        next += 1;
                    }
                    next
                };
                
                Solution::backtracking(nums, result, a, next, sum, target);

                if candidate {
                    a.pop();
                    sum -= nums[k];
                }
            }
        }
    }


}
