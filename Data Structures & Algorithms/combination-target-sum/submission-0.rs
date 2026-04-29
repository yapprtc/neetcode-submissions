impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let (mut result, mut a) = (vec![], vec![]);
        Solution::backtracking(&nums, &mut result, &mut a, 0, target);
        result
    }

    fn backtracking(nums:&[i32], result:&mut Vec<Vec<i32>>, a:&mut Vec<i32>, mut sum: i32, target: i32) {
        if sum > target {
            return;
        } else if sum == target {
            result.push(a.clone());
        } else {
            for (index, &candidate) in nums.iter().enumerate() {
                a.push(candidate);
                sum += candidate;
                
                Solution::backtracking(&nums[index..], result, a, sum, target);

                a.pop();
                sum -= candidate;
            }
        }
    }
}
