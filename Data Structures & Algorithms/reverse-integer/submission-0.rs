impl Solution {
    pub fn reverse(x: i32) -> i32 {
        //i32::max() =  2147483647
        //i32::min() = -2147483648
        if x == i32::MIN || x == 0 {
            return 0;
        }
        let (sign, mut x) = if x < 0 {
            (-1, -x)
        } else {
            (1, x)
        };
        
        let mut y = 0;
        //println!("x={}, y={}", x, y);
        while x != 0 {
            let d = x%10;
            if y == 214748364 && d > 7 {
                return 0;
            }
            y = (y * 10) + d;
            x /= 10;
            if y > 214748364 && x != 0 {
                return 0;
            }
            //println!("x={}, y={}", x, y);
        }

        if sign == - 1 {
            -y
        } else {
            y
        }
    }
}
