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

async function getInventory() {
  return await invoke("get_inv", { infile: inputTextEl.value });
}

async function inputButton() {
  const inventory = await getInventory();
  displayInventory(inventory);
}

async function displayInventory(inventoryArray) {
  inventoryTextEl.textContent = inventoryArray;
}

async function updateStock() {
  console.log("updating stock");
  displayInventory(
    await invoke("update_stock", {
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
    inputButton();
  });
  document
    .querySelector("#stock-change-form")
    .addEventListener("submit", (e) => {
      e.preventDefault();
      updateStock();
    });
});
