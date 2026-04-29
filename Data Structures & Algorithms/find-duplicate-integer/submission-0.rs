impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut i = 1i32;
        while i <= nums.len() as i32 {
            let next = nums[(i-1) as usize];
            if next == i {
                i += 1;
            } else if nums[(next-1) as usize] == next {
                return next;
            } else {
                nums.swap((i-1) as usize, (next-1) as usize);
            }
        }
        -1
    }
}
