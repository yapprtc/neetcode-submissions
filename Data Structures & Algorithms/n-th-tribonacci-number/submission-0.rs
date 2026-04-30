impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        } else if n <= 2 {
            return 1;
        }
        let (mut T0, mut T1, mut T2) = (0, 1, 1);
        let mut T3 = 0;
        for i in 3..=n {
            T3 = T0 + T1 + T2;
            T0 = T1;
            T1 = T2;
            T2 = T3; 
        }
        T3
    }
}
