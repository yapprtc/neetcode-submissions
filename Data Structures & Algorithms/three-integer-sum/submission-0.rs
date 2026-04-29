impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort(); // O(nlogn)
        let mut result = vec![];
        for k in 0..nums.len() { 
            if k > 0 && nums[k] == nums[k-1] {
                continue;
            }
            let (mut i, mut j) = (k as i32 + 1, nums.len() as i32 - 1);
            let target = -nums[k];
            while i < j {
                let sum = nums[i as usize] + nums[j as usize];
                if sum == target {
                    result.push(vec![nums[k], nums[i as usize], nums[j as usize]]);
                    i += 1;
                    j -= 1;
                    while i < j && nums[i as usize] == nums[(i - 1) as usize] {
                        i += 1;
                    } 
                    while i < j && nums[j as usize] == nums[(j + 1) as usize] {
                        j -= 1;
                    }
                } else if sum > target {
                    j -= 1;
                    while i < j && nums[j as usize] == nums[(j + 1) as usize] {
                        j -= 1;
                    }
                } else {
                    i += 1;
                    while i < j && nums[i as usize] == nums[(i - 1) as usize] {
                        i += 1;
                    } 
                }
            }
        }
        result
    }
}
