//import { invoke } from "@tauri-apps/api/tauri";
const { invoke } = window.__TAURI__.tauri;

// let greetInputEl;
// let greetMsgEl;

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("greet", { name: "1234" });
// }

let inputTextEl;
let inventoryTextEl;
let stockChangeNameEl;
let stockChangeAmtEl;

async function get_inventory() {
  inventoryTextEl.textContent = await invoke("get_inventory", {
    infile: inputTextEl.value,
  });
}

async function print_serialised_inv() {
  await invoke("print_serialised_inv", {
    serialisedInv: inventoryTextEl.textContent,
  });
}

window.addEventListener("DOMContentLoaded", () => {
  inputTextEl = document.querySelector("#input-text");
  inventoryTextEl = document.querySelector("#inventory");
  stockChangeNameEl = document.querySelector("#stock-change-name");
  stockChangeAmtEl = document.querySelector("#stock-change-amt");
  document.querySelector("#input-form").addEventListener("submit", (e) => {
    e.preventDefault();
    get_inventory();
  });
  document
    .querySelector("#stock-change-form")
    .addEventListener("submit", (e) => {
      e.preventDefault();
      print_serialised_inv();
    });
});
