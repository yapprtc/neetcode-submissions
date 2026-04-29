use std::collections::{HashMap, HashSet};

struct Twitter {
    follows: HashMap<i32, HashSet<i32>>,
    posts: HashMap<i32, Vec<(usize /*timestamp*/, i32/*tweet_id*/)>>,
    timestamp: usize,
}

impl Twitter {
    pub fn new() -> Self {
        Self {
            follows: HashMap::new(),
            posts: HashMap::new(),
            timestamp: 0,
        }
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.timestamp += 1;
        self.posts.entry(user_id)
            .and_modify(|v| v.push((self.timestamp, tweet_id)))
            .or_insert(vec![(self.timestamp, tweet_id)]);
        self.follows.entry(user_id)
            .or_insert({
                let mut v = HashSet::new();
                v.insert(user_id);
                v
            });
    }

    pub fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        if let Some(follows) = self.follows.get(&user_id) {
            let mut max_heap = BinaryHeap::new();
            for follow in follows.iter(){
                if let Some(posts) = self.posts.get(follow) {
                    if !posts.is_empty() {
                        let last = posts.len() - 1;
                        max_heap.push((posts[last].0, posts[last].1, *follow, last));
                    }
                }
            }
            let mut result = vec![];
            while let Some((_timestamp, tweet_id, follow, mut last)) = max_heap.pop() {
                if let Some(posts) = self.posts.get(&follow) && last != 0 {
                    last -= 1;
                    max_heap.push((posts[last].0, posts[last].1, follow, last));
                }
                result.push(tweet_id);
                if result.len() == 10 {
                    break;
                }
            }
            result
        } else {
            vec![]
        }
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id != followee_id {
            self.follows.entry(follower_id)
                .and_modify(|v| {v.insert(followee_id);})
                .or_insert({
                    let mut v = HashSet::new();
                    v.insert(follower_id);
                    v.insert(followee_id);
                    v
                });
        }
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id != followee_id {
            self.follows.entry(follower_id)
                .and_modify(|v| {v.remove(&followee_id);})
                .or_insert({
                    let mut v = HashSet::new();
                    v.insert(follower_id);
                    v
                });
        }
    }
}
