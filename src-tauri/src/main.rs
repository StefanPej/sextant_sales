// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let mut inventory = parse_input("input.txt");
    // update_input(&mut inventory, "Strongbox Enraged", -3);
    // println!("{:?}", inventory[0]);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_inventory,
            print_serialised_inv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Deserialize)]
struct Sextant {
    name: String,
    stock: i32,
    price: f32,
}

fn deserialise_inv(serialised_inv: &str) -> Vec<Sextant> {
    let inv_items: Vec<&str> = serialised_inv.split("-").collect();
    let mut inventory: Vec<Sextant> = Vec::new();

    for item in inv_items {
        let temp_sextant: Sextant = serde_json::from_str(item).unwrap();
        inventory.push(temp_sextant)
    }

    inventory
}

#[tauri::command]
fn get_inventory(infile: &str) -> String {
    let inventory = parse_input(infile);
    let mut serialized_inventory = Vec::new();

    for sextant in inventory {
        serialized_inventory.push(serde_json::to_string(&sextant).unwrap())
    }

    serialized_inventory.join("-")
}

#[tauri::command]
fn print_serialised_inv(serialised_inv: String) {
    let inventory = deserialise_inv(&serialised_inv);
    for sextant in inventory {
        println!("{:?}", sextant)
    }
}

fn update_input(inventory: &mut Vec<Sextant>, item_name: &str, stock_change: i32) {
    if let Some(item) = inventory.iter_mut().find(|item| item.name == item_name) {
        item.stock += stock_change;
    }
}

fn parse_input(infile: &str) -> Vec<Sextant> {
    let contents = fs::read_to_string(infile).expect("Can't read infile.");
    let mut inventory: Vec<Sextant> = Vec::new();

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

    inventory
}
