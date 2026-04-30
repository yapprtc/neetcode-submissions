impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
        while lo <= hi {
            if nums[lo as usize] == val {
                nums.swap(lo as usize, hi as usize);
                hi -= 1;
            } else {
                lo += 1;
            }
        }
        lo
    }
}
