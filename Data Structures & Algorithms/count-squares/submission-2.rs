use std::collections::HashMap;

struct CountSquares {
    points: HashMap<Vec<i32>, i32>,
}

impl CountSquares {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
        }
    }

    pub fn add(&mut self, point: Vec<i32>) {
        self.points.entry(point)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        let mut result = 0;
        for (p, c) in &self.points {
            if (p[0]-point[0]).abs() == (p[1]-point[1]).abs() && p[0] != point[0] {
                let p1 = vec![p[0], point[1]];
                let p2 = vec![point[0], p[1]];
                if let (Some(&c1), Some(&c2)) = (self.points.get(&p1), self.points.get(&p2)) {
                    //println!("c1={}, c2={}, p1={:?}, p2={:?}", c1, c2, p1, p2);
                    result += c * c1 * c2;
                }
            }
        }
        result
    }
}
