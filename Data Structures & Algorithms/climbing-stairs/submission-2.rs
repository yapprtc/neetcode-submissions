impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // f(n) = f(n-1) + f(n-2)
        if n <= 1 {
            return n;
        }

        let (mut f_1, mut f_2) = (1, 1);
        let mut f_0 = 0;
        for _ in 2..=n {
            f_0 = f_1 + f_2;
            f_1 = f_2;
            f_2 = f_0;
        }
        f_0
    }
}
