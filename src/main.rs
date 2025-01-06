mod cat_art;
use chrono::{Utc, Duration};
use std::io::{self};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;


#[derive(Serialize, Deserialize, Debug)]
struct Cat{
    name: String,
    hunger: usize,
    happiness: usize,
    bloodlust: usize
}

impl Cat {
    fn new(name: &str) -> Self{
        Self{
            name: name.to_string(),
            hunger: 10,
            happiness: 1,
            bloodlust: 10
        }
    }

    fn feed(&mut self){
        self.hunger = self.hunger.saturating_sub(2);
        self.happiness += 2;
        self.bloodlust = self.bloodlust.saturating_sub(2); //saturating sub ensures number doesn't go below 0
    }

    fn show_cat_stats(&mut self){
        println!("\n\n");
        println!("{}", self.name.to_uppercase());
        println!("Hunger: {}", "X".repeat(self.hunger));
        println!("Happiness: {}", "X".repeat(self.happiness));
        println!("Bloodlust: {}", "X".repeat(self.bloodlust));
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CatCollection{
    cats: Vec<Cat>
}

impl CatCollection{
    fn new() -> Self{
        Self{
            cats: Vec::new()
        }
    }

    fn add_cat(&mut self){
        let mut cat_name: String = String::new();

        println!("Enter cat name!");
    
        io::stdin()
        .read_line(&mut cat_name)
        .expect("Failed to read line");
        let cat_name=cat_name.trim();
        let new_cat = Cat::new(&cat_name);
        self.cats.push(new_cat);
        println!("{}", cat_art::CAT_CREATED);
        println!("{cat_name} has been created!\n\n");
    }

    fn view_cats(&mut self){
        for cat in &mut self.cats{
            cat.show_cat_stats();
        }
    }

    fn feed_cats(&mut self){
        for cat in &mut self.cats{
            cat.feed();
        }
        println!("{}", cat_art::CAT_EATING.repeat(self.cats.len()));
        println!("Your cats have been fed!\n\n");
    }

    fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string_pretty(&self)?;
        let mut file = File::create("cats.json")?;
        file.write_all(json_data.as_bytes())?;

        Ok(())
    }
}

fn read_cats_from_json() -> Result<CatCollection, Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string("cats.json")?;
    let cat_owner: CatCollection = serde_json::from_str(&file_content)?;
    Ok(cat_owner)
}

fn main() {
    let mut cat_collection = match read_cats_from_json() {
        Ok(collection) => collection,
        Err(_) => {
            println!("Failed to load cats from file. Starting with an empty collection.");
            CatCollection::new()  // Start with an empty collection if file reading fails
        }
    };
    

    loop {
        println!("Welcome to the virtual cat simulator! What would you like to do?\n");
        println!("{}",Utc::now());
        println!("1. Create a cat");
        println!("2. Show your cats");
        println!("3. Feed your cats");
        println!("4. Save and exit");

        let mut choice:String = String::new();

        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

        let choice:i8 = choice.trim().parse().unwrap();

        match choice {
            1 => cat_collection.add_cat(),
            2 => cat_collection.view_cats(),
            3 => cat_collection.feed_cats(),
            4 => {
                let _ = cat_collection.save();
                break;
            },
            _=> {
                println!("Select a valid option");
            }
        }
    }
}



// use chrono::{Utc, Duration};
// use std::{thread, time};

// struct Cat {
//     hunger: i32,      // Hunger level of the cat
//     created_at: chrono::DateTime<Utc>, // Timestamp of when the cat was created
// }

// impl Cat {
//     // Constructor to create a new Cat with a starting hunger value
//     fn new(hunger: i32) -> Self {
//         Cat {
//             hunger,
//             created_at: Utc::now(),
//         }
//     }

//     // Method to update the hunger level by decreasing it based on the number of days
//     fn update_hunger(&mut self) {
//         let now = Utc::now();
//         let days_since_creation = now.signed_duration_since(self.created_at).num_days();
        
//         // Decrease hunger by the number of days since the cat was created (if it's positive)
//         if days_since_creation > 0 {
//             self.hunger -= days_since_creation as i32;
//             // Ensure hunger doesn't go negative
//             if self.hunger < 0 {
//                 self.hunger = 0;
//             }
//         }
//     }

//     // Method to print the current state of the cat
//     fn display(&self) {
//         println!("Hunger: {}", self.hunger);
//         println!("Created on: {}", self.created_at);
//     }
// }

// fn main() {
//     let mut cat = Cat::new(100); // Starting hunger value is 100
    
//     loop {
//         // Print the current state of the cat
//         cat.display();

//         // Update hunger based on days passed
//         cat.update_hunger();
        
//         // Sleep for 24 hours (86400 seconds)
//         let one_day = time::Duration::from_secs(86400);
//         thread::sleep(one_day);
//     }
// }


// example to decrement a thing once per day

// use std::{thread, time};

// fn main() {
//     let mut number = 10; // Initialize the number (you can set this to any starting value)
    
//     loop {
//         println!("Current number: {}", number);
        
//         // Decrement the number
//         number -= 1;
        
//         // Check if number has reached 0
//         if number <= 0 {
//             println!("Number has reached zero or below. Exiting.");
//             break;
//         }

//         // Wait for 24 hours (86400 seconds)
//         let one_day = time::Duration::from_secs(86400); // 24 hours * 60 minutes * 60 seconds
//         thread::sleep(one_day);
//     }
// }

//TO SUBTRACT TIMES
// let duration = end_time.signed_duration_since(start_time);

// // Convert the duration to number of days
// let days = duration.num_days();

// To round down to number of whole cdays
// let days_rounded_down = duration.num_seconds() / 86400; // 86400 seconds in a day
