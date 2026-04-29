impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let (mut left, mut right) = (vec![1i32; l+1], vec![1i32; l+1]);
        
        for i in 0..l {
            left[i+1] = left[i] * nums[i];
        }
        for i in (0..l).rev() {
            right[i] = right[i+1] * nums[i];
        }
        let mut result = vec![0i32; l];
        for i in 0..l {
            result[i] = left[i]*right[i+1];
        }
        result
    }
}
