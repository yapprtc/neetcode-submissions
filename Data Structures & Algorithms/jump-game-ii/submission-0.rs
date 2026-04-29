impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut prev = vec![0; nums.len()];
        for i in 0..nums.len() {
            prev[i] = i;
        }
        let (mut i, mut j) = (0, std::cmp::min(0 + nums[0] as usize, nums.len() - 1));
        for k in i+1..=j {
            if i < prev[k] { 
                prev[k] = i;
            }
        }

        while i < j {
            i += 1;
            j = std::cmp::min(std::cmp::max(j, i + nums[i] as usize), nums.len() - 1);
            for k in i+1..=j {
                if i < prev[k] { 
                    prev[k] = i;
                }
            }
        }
        //println!("prev={:?}, i={}", prev, i);
        assert!(i == nums.len() - 1);
        let mut count = 0;
        while i != 0 {
            //println!("prev[i]={:?}, i={}", prev[i], i);
            i = prev[i];
            count += 1;
        }
        count
    }
}
