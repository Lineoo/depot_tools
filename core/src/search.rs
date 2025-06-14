use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

pub struct SearchEngine<T> {
    matcher: SkimMatcherV2,
    pool: Vec<(String, T)>,
    result: Vec<(i64, usize)>,
}
impl<T> SearchEngine<T> {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            pool: Vec::new(),
            result: Vec::new(),
        }
    }

    pub fn push(&mut self, name: String, value: T) {
        self.pool.push((name, value));
    }

    pub fn search(&mut self, pattern: &str) {
        let mut result = Vec::with_capacity(self.pool.len() / 4);
        for (i, (name, item)) in self.pool.iter().enumerate() {
            if let Some((score, indices)) = self.matcher.fuzzy_indices(name, pattern) {
                result.push((score, i));
            }
        }
        result.sort_by_key(|x| -x.0);
        self.result = result;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.result
            .get(index)
            .and_then(|x| self.pool.get(x.1))
            .map(|x| &x.1)
    }
}
