use loot::inventory::{Inventory, Optimize};
use loot::item::Item;

fn main() {
    println!("Running knapsack optimization benchmarks...\n");

    // Small inventory (8 items)
    let small_inventory: Inventory = vec![
        Item::new(95, 240),
        Item::new(88, 195),
        Item::new(92, 210),
        Item::new(75, 180),
        Item::new(82, 225),
        Item::new(90, 200),
        Item::new(70, 150),
        Item::new(85, 190),
    ];

    println!("Small inventory (8 items, limit 600):");
    for _ in 0..1000 {
        let result = small_inventory.optimal_value_with_weight_limit(600);
        std::hint::black_box(result);
    }
    println!("  Optimal value: {}", small_inventory.optimal_value_with_weight_limit(600));

    // Medium inventory (20 items)
    let medium_inventory: Inventory = (0..20)
        .map(|i| Item::new(50 + (i * 3) % 50, 100 + (i * 17) % 200))
        .collect();

    println!("\nMedium inventory (20 items, limit 1500):");
    for _ in 0..500 {
        let result = medium_inventory.optimal_value_with_weight_limit(1500);
        std::hint::black_box(result);
    }
    println!("  Optimal value: {}", medium_inventory.optimal_value_with_weight_limit(1500));

    // Large inventory (50 items)
    let large_inventory: Inventory = (0..50)
        .map(|i| Item::new(40 + (i * 7) % 60, 80 + (i * 23) % 300))
        .collect();

    println!("\nLarge inventory (50 items, limit 3000):");
    for _ in 0..100 {
        let result = large_inventory.optimal_value_with_weight_limit(3000);
        std::hint::black_box(result);
    }
    println!("  Optimal value: {}", large_inventory.optimal_value_with_weight_limit(3000));

    // Very large inventory (100 items)
    let very_large_inventory: Inventory = (0..100)
        .map(|i| Item::new(30 + (i * 11) % 70, 50 + (i * 29) % 400))
        .collect();

    println!("\nVery large inventory (100 items, limit 5000):");
    for _ in 0..10 {
        let result = very_large_inventory.optimal_value_with_weight_limit(5000);
        std::hint::black_box(result);
    }
    println!("  Optimal value: {}", very_large_inventory.optimal_value_with_weight_limit(5000));

    println!("\nBenchmarks complete!");
}
