use std::{ffi::c_void, io::{self}};

#[derive(Debug)]
struct Cat{
    name: String,
    hunger: i8
}

impl Cat {
    fn new(name: &str) -> Self{
        Self{
            name: name.to_string(),
            hunger: 6
        }
    }

    fn feed(&mut self){
        self.hunger = 0;
    }
}

#[derive(Debug)]
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

    fn view_cats(&self){
        println!("{:?}\n\n", self.cats);
    }

    fn feed_cats(&mut self){
        for cat in &mut self.cats{
            cat.feed();
        }
        println!("Your cats have been fed!\n\n");
    }
}

fn main() {
    let mut cat_collection = CatCollection::new();
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
            // 1 => new_cat(cat_collection),
            1 => cat_collection.add_cat(),
            2 => cat_collection.view_cats(),
            3 => cat_collection.feed_cats(),
            4 => break,
            _=> {
                println!("Select a valid option");
            }
        }
    }
}
