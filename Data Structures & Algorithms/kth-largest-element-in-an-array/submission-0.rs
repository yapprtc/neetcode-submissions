impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let k = nums.len() as i32 - k + 1;
        Solution::find_kth_smallest_helper(&mut nums, k)
    }

    fn find_kth_smallest_helper(nums: &mut [i32], k: i32) -> i32 {
        //println!("nums={:?}, k={}", nums, k);
        let (i, j, l) = Solution::find_kth_smallest(nums);
        //println!("i={}, j={}, l={}, nums={:?}", i, j, l, nums);
        if k <= i {
            Solution::find_kth_smallest_helper(&mut nums[..i as usize], k)
        } else if k <= j {
            nums[i as usize]
        } else {
            Solution::find_kth_smallest_helper(&mut nums[j as usize..], k - j)
        }
    }

    fn find_kth_smallest(nums: &mut [i32]) -> (i32, i32, i32) {
        let (mut i, mut j, mut l) = (0i32, 0i32, nums.len() as i32 - 1);
        while j <= l {
            if nums[j as usize] == nums[i as usize] {
                j += 1;
            } else if nums[j as usize] < nums[i as usize] {
                nums.swap(i as usize, j as usize);
                i += 1;
                j += 1;
            } else {
                nums.swap(j as usize, l as usize);
                l -= 1;
            }
        }
        (i, j, l)
    }
}
