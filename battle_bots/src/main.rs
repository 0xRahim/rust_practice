/*
### 2. "Battle Bots" Simulation

This is the ultimate test for **Traits and Pattern Matching**. You’ll build a text-based arena where different "entities" fight each other.

* **The Goal:** Define a `Fighter` trait and implement it for different structs (e.g., `Robot`, `Warrior`, `Wizard`).
* **Concepts to Practice:**
* **Traits:** Define a trait with methods like `attack()` and `take_damage()`.
* **Generics:** Write a function `start_duel<T: Fighter, U: Fighter>(p1: T, p2: U)` that runs the combat loop.
* **Pattern Matching:** Use `match` to handle different combat outcomes or special abilities.
* **Ownership:** Move fighters into the arena and decide if the winner is returned (transferred back) or if they all stay in a `Vector`.
*/

enum Skills {
    Shoot,
    Dodge,
    TakeCover,
}
struct Robot {
    health: i32,
    Skill: Skills,
}
struct Warrior {
    health: i32,
    Skill: Skills,
}
struct Wizard {
    health: i32,
    Skill: Skills,
}
trait Fighter {
    fn attack<T: Fighter>(&self, target: &mut T);
    fn take_damage(&mut self, damage: i32);
    fn get_health(&self)-> i32;
}

impl Fighter for Robot {
    fn attack<T: Fighter>(&self, target: &mut T) {
        target.take_damage(10);
    }
    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
    fn get_health(&self) -> i32{
        self.health
    }
}

impl Fighter for Warrior{
    fn attack<T: Fighter>(&self, target: &mut T) {
        target.take_damage(5);
    }
    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
    fn get_health(&self) -> i32{
        self.health
    }
}

fn start_duel<T: Fighter, U: Fighter>(p1: T, p2: U) {
    let mut p1 = p1;
    let mut p2 = p2;   
    loop{
        p1.attack(&mut p2);
        p2.attack(&mut p1);
        let p1_health = p1.get_health();
        let p2_health = p2.get_health();

        if p1_health <= 0 {
            println!("Player 2 wins!");
            break;
        } else if p2_health <= 0 {
            println!("Player 1 wins!");
            break;
        }
    }
}
fn main() {
    let robot = Robot { health: 100, Skill: Skills::Shoot };
    let warrior = Warrior { health: 100, Skill: Skills::Dodge };
    start_duel(robot, warrior); 
}
