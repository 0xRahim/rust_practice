struct Player {
    name: String,
    gold: u32,
    inventory: Vec<String>,
}

fn main() {
    let mut player = Player {
        name: String::from("Aris"),
        gold: 50,
        inventory: vec![String::from("Rusty Sword")],
    };

    let rewards = vec![
        ("Healing Potion", 10),
        ("Golden Shield", 100),
        ("Small Gem", 20),
        ("Excalibur", 500),
    ];

    // --- TASK 1: Fn (Immutable Borrow) ---
    // Create a closure 'can_afford' that captures the player's current gold.
    // It should take a reward price (u32) and return true if gold >= price.
    // Task 1 shows how closures make Iterators powerful by letting them "see" your local variables.
    let can_afford = |price|->bool{
        if player.gold>=price{
            return true;
        }
        else{
            return false;
        }
    };

    let affordable_items: Vec<_> = rewards
        .iter()
        .filter(|(_, price)| can_afford(*price))
        .collect();

    println!("Affordable items: {:?}", affordable_items);


    // --- TASK 2: FnMut (Mutable Borrow) ---
    // Create a closure 'buy_item' that captures a MUTABLE reference to the player.
    // It should take an item name (&str) and a price (u32).
    // Inside: subtract price from gold and push the name to player.inventory.
    // Task 2 shows how closures help organize code by pre-packaging logic with the specific data (the player) it needs to operate on.
    let mut buy_item = |item:&str,price|{
        player.gold-=price;
        player.inventory.push(String::from(item));
    };

    buy_item("Small Gem", 20);
    println!("After buying gem: {} gold, inventory: {:?}", player.gold, player.inventory);


    // --- TASK 3: FnOnce (Ownership/Move) ---
    // We have a 'secret_message' that we want to print once and then destroy.
    let secret_message = String::from("The treasure is under the bridge!");

    // Create a closure 'reveal_secret' that uses the 'move' keyword 
    // to take ownership of secret_message and print it.
    let reveal_secret = move || {
        println!("Secret message: {}", secret_message)
    };

    reveal_secret();
    // println!("{}", secret_message); // <--- This should fail if you uncomment it!
}