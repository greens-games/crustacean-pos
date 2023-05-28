use std::{fs::File, collections::HashMap, cmp::Ordering};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};


/*
TODO: 
Change ordering to be a list of items and user enters number to add to their order
format order into (receipt/kitchen ticket)
 */

#[derive(Deserialize, Serialize)]
struct MenuItem {
    name: String,
    price: f32,
}

impl MenuItem {
    pub fn new(name: String, price: f32) -> Self {
        MenuItem {
            name,
            price
        }
    }

    pub fn to_string(&self) -> String {
        format!("Item ordered: {}; Price of Item: {}", &self.name, &self.price)
    }
}

fn main() {
    //parse json file
    let menu_items: HashMap<String,MenuItem> = serde_json::from_reader(File::open("res/menu.json").unwrap()).unwrap();
    //load parsed data into memory (data structure of some sort)
    //allow to order to be placed
    process_order(ask_for_order(menu_items));
}

fn process_order(placed_order: Vec<MenuItem>) {
    for item in placed_order {
        println!("{}",item.to_string());
    } 
}

fn ask_for_order(menu: HashMap<String, MenuItem>) -> Vec<MenuItem> {
    let mut input = String::new();
    let mut menuBoard = String::new();
    println!("Menu (Enter number to add to order):");
    let mut order: Vec<MenuItem> = Vec::new();
    //print list of items numbered have 0 be done
    for menu_item in &menu {
        menuBoard.push_str(menu_item.0);
        menuBoard.push_str(": ");
        menuBoard.push_str(&menu_item.1.name);
        menuBoard.push_str("\n");
    }
    menuBoard.push_str("Enter 0 when done ordering");
    println!("{}", menuBoard);
    while !input.trim().eq("0") {
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Invalid string");
        match menu.get(input.trim()) {
            Some(item) => order.push(MenuItem::new(item.name.clone(), item.price)),
            None => println!("Invalid option")
        }
    }
    order 
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{MenuItem, process_order};

    #[test]
    fn process_order_test() {
        let order = "Spaghetti".to_string();
        let menu_item = MenuItem {
            name: ("Spaghetti".to_string()),
            price: (2.00)
        };
        let map = HashMap::from([("Spaghetti".to_string(),menu_item)]);
        
    }
}