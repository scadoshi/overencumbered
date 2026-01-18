pub mod inventory;
pub mod item;

#[cfg(test)]
mod integration_tests {
    use crate::inventory::{Inventory, Optimize};
    use crate::item::Item;

    #[test]
    fn test_integration_optimize_workflow() {
        let item1 = Item::new(95, 240);
        let item2 = Item::new(85, 180);
        let item3 = Item::new(75, 200);
        let item4 = Item::new(90, 210);
        let item5 = Item::new(65, 150);

        let my_inventory: Inventory = vec![item1, item2, item3, item4, item5]
            .into_iter()
            .collect();

        let limit = 420;
        let optimal_value = my_inventory.optimal_value_with_weight_limit(limit);

        assert_eq!(optimal_value, 180);
    }

    #[test]
    fn test_integration_edge_cases() {
        let empty: Inventory = vec![].into_iter().collect();
        let optimal_value_empty = empty.optimal_value_with_weight_limit(300);
        assert_eq!(optimal_value_empty, 0);

        let single_fits: Inventory = vec![Item::new(80, 100)].into_iter().collect();
        let optimal_value_single = single_fits.optimal_value_with_weight_limit(200);
        assert_eq!(optimal_value_single, 80);

        let single_too_long: Inventory = vec![Item::new(90, 300)].into_iter().collect();
        let optimal_value_too_long = single_too_long.optimal_value_with_weight_limit(100);
        assert_eq!(optimal_value_too_long, 0);
    }
}
