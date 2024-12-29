use std::io::{self};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

fn handle_negative(number: usize) -> usize{
    if number < 0{
        0
    } else{
        number
    }
}

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
        self.hunger +=2;
        self.happiness += 2;
        self.bloodlust -= 2;
        // let temp = self.bloodlust;
        // self.bloodlust = handle_negative(temp);
    }

    fn show_cat_stats(&mut self){
        println!("\n\n");
        println!("{:?}", self.name.to_uppercase());
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
        print!("{cat_name} has been created!\n\n");
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
    // Read the file contents into a string
    let file_content = std::fs::read_to_string("cats.json")?;

    // Deserialize the JSON string into a CatOwner object
    let cat_owner: CatCollection = serde_json::from_str(&file_content)?;
    Ok(cat_owner)
}

fn main() {
    // let mut cat_collection = CatCollection::new();

    // let cat_collection = read_cats_from_json();
    // println!("TEST: {:?}", test);
    let mut cat_collection = match read_cats_from_json() {
        Ok(collection) => collection,
        Err(_) => {
            println!("Failed to load cats from file. Starting with an empty collection.");
            CatCollection::new()  // Start with an empty collection if file reading fails
        }
    };
    

    loop {
        println!("Welcome to the virtual cat simulator! What would you like to do?\n");
        println!("1. Create a cat");
        println!("2. Show your cats");
        println!("3. Feed your cats");
        println!("4. Exit");

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
