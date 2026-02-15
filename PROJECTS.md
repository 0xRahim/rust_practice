1) Linked List Code 
2) Stack And Queue Code 


### 1. The "Rusty Inventory" Manager (CLI)

This project focuses on **Modules, Collections, and Error Handling**. Instead of a simple "To-Do" list, build a CLI tool that manages a store inventory.

* **The Goal:** A program where you can add, remove, and update stock items.
* **Concepts to Practice:**
* **HashMaps:** Store items using a unique ID (String) as a key and a `Struct` as the value.
* **Enums:** Use an enum for "Categories" (e.g., `Electronics`, `Food`, `Clothing`).
* **Error Handling:** Use `Result` to handle cases where an item ID doesn't exist or a user enters a negative stock number.
* **Modules:** Split your logic into `inventory.rs` and `main.rs`.



### 2. "Battle Bots" Simulation

This is the ultimate test for **Traits and Pattern Matching**. You’ll build a text-based arena where different "entities" fight each other.

* **The Goal:** Define a `Fighter` trait and implement it for different structs (e.g., `Robot`, `Warrior`, `Wizard`).
* **Concepts to Practice:**
* **Traits:** Define a trait with methods like `attack()` and `take_damage()`.
* **Generics:** Write a function `start_duel<T: Fighter, U: Fighter>(p1: T, p2: U)` that runs the combat loop.
* **Pattern Matching:** Use `match` to handle different combat outcomes or special abilities.
* **Ownership:** Move fighters into the arena and decide if the winner is returned (transferred back) or if they all stay in a `Vector`.



### 3. A Basic Log Parser (The "Lifetime" Boss)

If you want to truly test your understanding of **Lifetimes and Strings**, this is the one.

* **The Goal:** Read a multi-line string (simulating a log file) and extract "Warnings" and "Errors" into a structured format without copying the data unnecessarily.
* **Concepts to Practice:**
* **Lifetimes:** Create a struct `LogEntry<'a>` that holds a reference (`&'a str`) to a slice of the original log string. This prevents heap allocations and keeps it fast.
* **Slices:** Use string slicing to find keywords like `[ERROR]` or `[INFO]`.
* **Vectors:** Store your `LogEntry` structs in a `Vec`.
* **Iterators:** (Sneak peek for Chapter 13) Use `.lines()` to loop through the input.



