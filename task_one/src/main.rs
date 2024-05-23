mod lib.rs;



struct Character {
    name: String,
    core: u32, 
    level: u32,
}

impl Character {
    fn new(name: &str, core: u32, level: u32) -> Character {
        Character {
            name: name.to_string(), 
            core,
            level,
        }
    }

    fn attack(&self, target: &mut Character) {
        // Simulate an attack, 
        println!("{} attacks {}!", self.name, target.name);
        target.core -= 5; // Reduce target's core by a fixed amount (adjust as needed)
    }

    fn display_info(&self) {
        println!("Name: {}", self.name);
        println!("Core: {}", self.core);
        println!("Level: {}", self.level);
    }
}

fn main() {
    let mut player = Character::new("Solanke", 100, 3);
    let mut monster = Character::new("Kepa", 50, 1);

    player.attack(&mut monster);
    monster.display_info(); // Check monster's core after attack

    player.display_info(); // Display player information
}

