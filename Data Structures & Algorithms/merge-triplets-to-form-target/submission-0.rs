impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut triplets = triplets;
        triplets.retain(|triplet| triplet[0] <= target[0] && triplet[1] <= target[1] && triplet[2] <= target[2]);
        triplets.iter().any(|triplet| triplet[0] == target[0]) && 
        triplets.iter().any(|triplet| triplet[1] == target[1]) &&
        triplets.iter().any(|triplet| triplet[2] == target[2])
    }
}
