// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{ser, Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::State;

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Vec::<Sextant>::new()))
        .invoke_handler(tauri::generate_handler![get_inv, update_stock])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Sextant {
    name: String,
    stock: i32,
    price: f32,
}

#[tauri::command]
fn get_inv(infile: &str, inventory_state: State<'_, Mutex<Vec<Sextant>>>) -> Vec<String> {
    let contents = fs::read_to_string(infile).expect("Can't read infile.");
    let mut inventory = inventory_state.lock().unwrap();

    // Remove everything in inventory
    // TODO: surely there has to be a way to just reinitialise an empty vector?
    for _i in 0..inventory.clone().len() {
        inventory.remove(0);
    }

    // Add everything to inventory
    for line in contents.lines() {
        if line.chars().next().unwrap().is_alphabetic() {
            continue;
        }

        let info = line.split(" / ").next().unwrap();

        let parts: Vec<&str> = info.split(" ").collect();
        let len_parts = parts.len();

        let temp_sextant = Sextant {
            name: parts[1..len_parts - 1].join(" "),
            stock: parts[0].replace("x", "").parse::<i32>().unwrap(),
            price: parts[len_parts - 1]
                .replace("c", "")
                .parse::<f32>()
                .unwrap(),
        };

        inventory.push(temp_sextant)
    }
    serialise_inv(inventory.clone())
}

#[tauri::command]
fn update_stock(
    inventory_state: State<'_, Mutex<Vec<Sextant>>>,
    item_names: Vec<&str>,
    stock_change: &str,
) -> Vec<String> {
    let mut inventory = inventory_state.lock().unwrap();
    let mut stock_change_int = match stock_change.parse::<i32>() {
        Ok(num) => num,
        Err(err) => return serialise_inv(inventory.clone()),
    };

    stock_change_int = stock_change_int * -1;

    for item_name in item_names {
        if let Some(item) = inventory.iter_mut().find(|item| item.name == item_name) {
            item.stock += stock_change_int;
        }
    }

    serialise_inv(inventory.clone())
}

fn deserialise_inv(serialised_inv: &str) -> Vec<Sextant> {
    let inv_items: Vec<&str> = serialised_inv.split("|").collect();
    let mut inventory: Vec<Sextant> = Vec::new();

    for item in inv_items {
        let temp_sextant: Sextant = serde_json::from_str(item).unwrap();
        inventory.push(temp_sextant)
    }

    inventory
}

fn serialise_inv(inventory: Vec<Sextant>) -> Vec<String> {
    let mut serialized_inventory = Vec::new();
    for sextant in inventory {
        serialized_inventory.push(serde_json::to_string(&sextant).unwrap())
    }

    serialized_inventory
}

fn print_serialised_inv(serialised_inv: String) {
    let inventory = deserialise_inv(&serialised_inv);
    for sextant in inventory {
        println!("{:?}", sextant)
    }
}
