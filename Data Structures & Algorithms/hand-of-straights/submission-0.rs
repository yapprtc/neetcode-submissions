impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let mut map = BTreeMap::new();
        let mut count = 0;
        for h in hand {
            map.entry(h)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            count += 1;
        }
        let mut hand:Vec<(i32, i32)> = map.into_iter().collect();
        let mut i = 0;
        while count > 0 {
            if i+group_size as usize > hand.len() {
                return false;
            }
            for j in i..i+group_size as usize {
                if j > i && (hand[j].0 != hand[j-1].0+1 || hand[j].1 == 0) {
                    return false;
                }
                hand[j].1 -= 1;
                count -= 1;
            }
            while i < hand.len() && hand[i].1 == 0 {
                i += 1;
            }
        }
        true
    }
}
