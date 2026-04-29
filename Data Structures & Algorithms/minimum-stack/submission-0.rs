struct MinStack {
    primary: Vec<i32>,
    secondary: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            primary: vec![],
            secondary: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        if self.primary.is_empty() || val <= self.secondary[self.secondary.len() - 1]{
            self.secondary.push(val);
        }
        self.primary.push(val);
    }

    pub fn pop(&mut self) {
        assert!(!self.primary.is_empty());
        if self.primary.pop().unwrap() == self.secondary[self.secondary.len() - 1] {
            self.secondary.pop();
        }
    }

    pub fn top(&self) -> i32 {
        assert!(!self.primary.is_empty());
        self.primary[self.primary.len() - 1]
    }

    pub fn get_min(&self) -> i32 {
        assert!(!self.secondary.is_empty());
        self.secondary[self.secondary.len() - 1]
    }
}
