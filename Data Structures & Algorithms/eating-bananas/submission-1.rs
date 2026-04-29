impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        assert!(!piles.is_empty());
        let (mut lo, mut hi) = (1, *piles.iter().max().unwrap());
        while lo <= hi {
            let mi = lo + (hi - lo) / 2;
            let hours = Solution::eating_hours(&piles, mi);
            //println!("lo={}, mi={}, hi={}, h={}, hours={}", lo, mi, hi, h, hours);
            if hours <= h as i64 {
                hi = mi - 1;
            } else {
                lo = mi + 1;
            }
        }
        lo
    }

    fn eating_hours(piles: &[i32], speed: i32) -> i64 {
        let mut hours = 0;
        for &pile in piles {
            hours += (pile as i64 +  speed as i64 - 1)/ (speed as i64);
        }
        hours
    }
}
