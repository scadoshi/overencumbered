use crate::item::Item;

pub type Inventory = Vec<Item>;

pub trait Optimize {
    fn optimal_value_with_weight_limit(&self, limit: usize) -> usize;
    fn optimize_with_weight_limit(self, limit: usize) -> Self;
}

impl Optimize for Inventory {
    fn optimal_value_with_weight_limit(&self, limit: usize) -> usize {
        let item_count = self.len();
        let mut dp = vec![vec![0; limit + 1]; item_count + 1];
        (1..=item_count).for_each(|i| {
            (0..=limit).for_each(|w| {
                dp[i][w] = dp[i - 1][w];
                if self[i - 1].weight() <= w {
                    dp[i][w] =
                        dp[i][w].max(dp[i - 1][w - self[i - 1].weight()] + self[i - 1].value());
                }
            })
        });
        dp[item_count][limit]
    }

    fn optimize_with_weight_limit(self, limit: usize) -> Self {
        let item_count = self.len();
        let mut dp = vec![vec![0; limit + 1]; item_count + 1];
        (1..=item_count).for_each(|i| {
            (0..=limit).for_each(|w| {
                dp[i][w] = dp[i - 1][w];
                if self[i - 1].weight() <= w {
                    dp[i][w] =
                        dp[i][w].max(dp[i - 1][w - self[i - 1].weight()] + self[i - 1].value());
                }
            })
        });

        let mut w = limit;
        let selections = (1..=item_count).rev().fold(Vec::new(), |mut acc, i| {
            if dp[i][w] != dp[i - 1][w] {
                acc.push(i - 1);
                w -= self[i - 1].weight();
            }
            acc
        });
        self.into_iter()
            .enumerate()
            .filter(|(i, _)| selections.contains(i))
            .map(|(_, v)| v)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_all_items_fit() {
        let inventory: Inventory = vec![Item::new(80, 180), Item::new(60, 120), Item::new(70, 150)]
            .into_iter()
            .collect();

        let limit = 500;
        let optimal_value = inventory.optimal_value_with_weight_limit(limit);

        assert_eq!(optimal_value, 210);
    }

    #[test]
    fn test_optimize_must_choose_subset() {
        let inventory: Inventory = [
            Item::new(100, 300),
            Item::new(90, 200),
            Item::new(50, 100),
            Item::new(80, 250),
        ]
        .into_iter()
        .collect();

        let limit = 400;
        let optimal_value = inventory.optimal_value_with_weight_limit(limit);

        assert_eq!(optimal_value, 150);
    }

    #[test]
    fn test_optimize_greedy_trap() {
        let inventory: Inventory = [Item::new(100, 201), Item::new(99, 200), Item::new(99, 200)]
            .into_iter()
            .collect();

        let limit = 400;
        let optimal_value = inventory.optimal_value_with_weight_limit(limit);

        assert_eq!(optimal_value, 198);
    }

    #[test]
    fn test_optimize_no_items_fit() {
        let inventory: Inventory = [Item::new(90, 300), Item::new(80, 400), Item::new(70, 250)]
            .into_iter()
            .collect();

        let limit = 100;
        let optimal_value = inventory.optimal_value_with_weight_limit(limit);

        assert_eq!(optimal_value, 0);
    }

    #[test]
    fn test_optimize_realistic_inventory() {
        let inventory: Inventory = [
            Item::new(95, 240),
            Item::new(88, 195),
            Item::new(92, 210),
            Item::new(75, 180),
            Item::new(82, 225),
            Item::new(90, 200),
            Item::new(70, 150),
            Item::new(85, 190),
        ]
        .into_iter()
        .collect();

        let limit = 600;
        let optimal_value = inventory.optimal_value_with_weight_limit(limit);

        assert_eq!(optimal_value, 267);
    }

    #[test]
    fn test_optimize_exact_fit() {
        let inventory: Inventory = [Item::new(80, 100), Item::new(90, 150), Item::new(70, 50)]
            .into_iter()
            .collect();

        let limit = 300;
        let optimal_value = inventory.optimal_value_with_weight_limit(limit);

        assert_eq!(optimal_value, 240);
    }
}
