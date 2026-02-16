pub mod inventory {
    use std::io::{self, Write};
    use std::{collections::HashMap, io::StdinLock};
    pub struct Item {
        pub name: String,
        pub category: Category,
        pub stock: u32,
        pub price: f32,
    }

    #[derive(Debug)]
    pub enum Category {
        Electronics,
        Food,
        Clothing,
    }
    impl Item {
        pub fn new() -> Self {
            let mut name = String::new();
            let mut category = String::new();
            let mut stock = String::new();
            let mut price = String::new();
            println!("Enter the name of the item:");
            std::io::stdin().read_line(&mut name).unwrap();
            println!("Enter the category of the item:");
            std::io::stdin().read_line(&mut category).unwrap();

            let category: Category = match category.trim().to_lowercase().as_str() {
                "electronics" => Category::Electronics,
                "food" => Category::Food,
                "clothing" => Category::Clothing,
                _ => {
                    print!("Invalid category.");
                    std::process::exit(1);
                }
            };

            println!("Enter the stock of the item:");
            std::io::stdin().read_line(&mut stock).unwrap();
            let stock: u32 = stock.trim().parse().expect("Invalid Stock Value");

            println!("Enter the price of the item:");
            std::io::stdin().read_line(&mut price).unwrap();
            let price: f32 = price.trim().parse().expect("Invalid price entered");
            Item {
                name,
                category,
                stock,
                price,
            }
        }
    }

    pub fn display_inventory(inventory: &HashMap<String, Item>) {
        println!("---------------------------------------");
        for (name, item) in inventory {
            println!("---------------------------------------");
            println!("*Name: {}", name);
            println!("  [-] Category: {:?}", item.category);
            println!("  [-] Stock: {}", item.stock);
            println!("  [-] Price: ${}", item.price);
            println!("---------------------------------------");
        }
    }

    pub fn print_menu() {
        println!("=================================");
        println!("  | RUSTY INVENTORY MANAGER |");
        println!("=================================");
        println!("  1. Add Item");
        println!("  2. Remove Item");
        println!("  3. Update Item");
        println!("  4. View Inventory");
        println!("  5. Exit");
        println!("  6. Menu");
        println!("=================================");
    }

    pub fn delete_item(inventory: &mut HashMap<String, Item>, name: &str) {
        inventory.remove(name);
    }
    
}
