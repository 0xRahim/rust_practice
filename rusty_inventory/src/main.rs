/*

### 1. The "Rusty Inventory" Manager (CLI)

This project focuses on **Modules, Collections, and Error Handling**. Instead of a simple "To-Do" list, build a CLI tool that manages a store inventory.

* **The Goal:** A program where you can add, remove, and update stock items.
* **Concepts to Practice:**
* **HashMaps:** Store items using a unique ID (String) as a key and a `Struct` as the value.
* **Enums:** Use an enum for "Categories" (e.g., `Electronics`, `Food`, `Clothing`).
* **Error Handling:** Use `Result` to handle cases where an item ID doesn't exist or a user enters a negative stock number.
* **Modules:** Split your logic into `inventory.rs` and `main.rs`.
*/

use std::{collections::HashMap, io::StdinLock};
use std::io::{self, Write};
use rusty_inventory::inventory::*;



fn main() {
    print_menu();
    let mut inventory: HashMap<String, Item> = HashMap::new();
    loop {
        let mut choice = String::new();
        print!("INPUT >> ");
        io::stdout().flush().unwrap();  
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        match choice {
            1 => {
                println!("Adding an item..");
                println!("-------------------");
                let item = Item::new();
                inventory.insert(item.name.clone(), item);
                println!("Item added successfully!");
                println!("-------------------");
                print!("\x1B[2J\x1B[H");
                print_menu();
            }
            2 => {
                println!("Removing an item..");
                println!("-------------------");
                let mut name = String::new();
                println!("Enter the name of the item to remove:");
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();
                delete_item(&mut inventory, name);
                println!("Item removed successfully!");
                println!("-------------------");
                print!("\x1B[2J\x1B[H");
                print_menu();
            }
            3 => {
                println!("Updating an item..");
                println!("-------------------");
                let mut name = String::new();
                println!("Enter the name of the item to update:");
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();
                delete_item(&mut inventory, name);
                let updated_item = Item::new();
                inventory.insert(updated_item.name.clone(), updated_item);  
                println!("Item updated successfully!");
                println!("-------------------");

                print!("\x1B[2J\x1B[H");
                print_menu();
            }
            4 => {
                println!("Viewing inventory..");
                println!("-------------------");
                display_inventory(&inventory);
                
                print_menu();
            }
            5 => {
                println!("Exiting the program.");
                break;
            }
            6 => {
                println!("Menu.");
                print_menu();
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
