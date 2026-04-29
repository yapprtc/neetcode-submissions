impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut global = 0;
        let (mut i, mut j) = (0i32, heights.len() as i32 - 1);
        while i < j {
            let local = (j - i) * std::cmp::min(heights[i as usize], heights[j as usize]);
            global = std::cmp::max(local, global);
            if  heights[i as usize] < heights[j as usize] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        global
    }
}
