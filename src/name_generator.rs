pub struct NameGenerator {
    current: usize,
}

impl NameGenerator {
    pub fn new() -> Self {
        Self { current: 1 }
    }
}

impl Iterator for NameGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Some(self.current.to_string());
        self.current += 1;
        out
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_name_generator() {
        let mut generator = NameGenerator::new();
        assert!(generator.next().is_some());

        const CNT: usize = 10_000;
        let set = generator.take(CNT).collect::<HashSet<_>>();
        assert_eq!(set.len(), CNT);
    }
}
