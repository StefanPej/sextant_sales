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
  display_inv(await invoke("get_inventory", { infile: inputTextEl.value }));
}

function display_inv(inventoryString) {
  inventoryTextEl.textContent = inventoryString;
}

async function update_stock() {
  console.log("updating stock");
  display_inv(
    await invoke("update_stock", {
      serialisedInv: inventoryTextEl.textContent,
      itemName: stockChangeNameEl.value,
      stockChange: stockChangeAmtEl.value,
    })
  );
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
      update_stock();
    });
});
