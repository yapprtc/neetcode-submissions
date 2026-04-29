impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
        while lo < hi {
            if nums[lo as usize] < nums[hi as usize] {
                return nums[lo as usize];
            }
            let mi = lo + (hi - lo) / 2;
            if nums[mi as usize] >= nums[lo as usize] {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        nums[lo as usize]
    }
}
