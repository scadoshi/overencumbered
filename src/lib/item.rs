#[derive(Debug)]
pub struct Item {
    value: usize,
    weight: usize,
}

impl Item {
    pub fn new(value: usize, weight: usize) -> Self {
        Self { value, weight }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn weight(&self) -> usize {
        self.weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let value = usize::try_from(75).unwrap();
        let item = Item::new(value, 180);
        assert_eq!(item.value(), 75);
        assert_eq!(item.weight(), 180);
    }
}
