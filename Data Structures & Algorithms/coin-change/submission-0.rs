impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // A[W] = min(A[W-Wj]+1), j in 0..n
        if amount < 0 {
            return -1;
        } else if amount == 0 {
            return 0;
        }
        assert!(amount>0);
        let mut A = vec![i32::MAX - 1 ; amount as usize + 1];
        A[0] = 0;
        for w in 1..amount as usize + 1 {
            for j in 0..coins.len() {
                if coins[j] <= w as i32 {
                    //println!("A[{}]={}, A[w-coins[{}] as usize]={}", w, A[w], j, A[w-coins[j] as usize]);
                    A[w] = std::cmp::min(A[w], A[w-coins[j] as usize]+1);
                }
            }
        }
        //println!("A={:?}", A);
        if A[amount as usize] == i32::MAX - 1 {
            -1
        } else {
            A[amount as usize]
        }
    }
}
