// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use glob::glob;
use serde::{ser, Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::MAIN_SEPARATOR_STR;
use std::sync::Mutex;
use tauri::State;

fn main() {
    make_icon_hashmap();
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
    icon: String,
}

#[tauri::command]
fn get_inv(infile: &str, inventory_state: State<'_, Mutex<Vec<Sextant>>>) -> Vec<String> {
    let full_infile_path = format!("{}{}", "./assets/", infile);
    let contents = fs::read_to_string(full_infile_path).expect("Can't read infile.");
    let mut inventory = inventory_state.lock().unwrap();
    let name_maps = make_icon_hashmap();

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

        let sextant_name = parts[1..len_parts - 1].join(" ");
        //println!("{}", sextant_name);
        let sextant_icon = get_icon_filename(&name_maps[&sextant_name]);

        let temp_sextant = Sextant {
            name: parts[1..len_parts - 1].join(" "),
            stock: parts[0].replace("x", "").parse::<i32>().unwrap(),
            price: parts[len_parts - 1]
                .replace("c", "")
                .parse::<f32>()
                .unwrap(),
            icon: sextant_icon,
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

fn serialise_inv(inventory: Vec<Sextant>) -> Vec<String> {
    let mut serialized_inventory = Vec::new();
    for sextant in inventory {
        serialized_inventory.push(serde_json::to_string(&sextant).unwrap())
    }

    serialized_inventory
}

fn get_icon_filename(icon_name: &str) -> String {
    let mut matches = Vec::<String>::new();
    for res in glob(format!("../src/assets/*{}*", icon_name).as_str())
        .expect("Failed to read glob pattern")
    {
        let filepath = res.unwrap().into_os_string().into_string().unwrap();
        matches.push(filepath);
    }

    if matches.len() == 1 {
        let filename_list: Vec<_> = matches[0].split(MAIN_SEPARATOR_STR).collect();
        let ret_val = filename_list[filename_list.len() - 1].to_owned();

        //println!("{}", ret_val);
        return ret_val;
    }
    "dib_think.png".to_owned()
}

fn make_icon_hashmap() -> HashMap<String, String> {
    let contents = fs::read_to_string("./assets/sextant_icons.csv").expect("Can't read infile.");

    let mut name_maps: HashMap<String, String> = HashMap::new();

    for line in contents.lines() {
        let split_line: Vec<_> = line.split(",").collect();
        name_maps.insert(split_line[0].to_string(), split_line[1].to_string());
    }

    name_maps
}
