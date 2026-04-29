impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            if nums[mi as usize] < nums[hi as usize] {
                hi = mi;
            } else {
                lo = mi + 1;
            }
        }
        nums[lo as usize]
    }
}
