const { invoke } = window.__TAURI__.tauri;

let inputTextEl;
let inventoryTextEl;
let stockChangeNameEl;
let stockChangeAmtEl;
let selectedSextants = [];

async function getInventory() {
  return await invoke("get_inv", { infile: inputTextEl.value });
}

async function getInventoryFromTxt() {
  const inventory = await getInventory();
  createInventoryButtons(inventory);
}

async function updateStock() {
  console.log("updating stock");
  const updatedInv = await invoke("update_stock", {
    itemNames: selectedSextants,
    stockChange: stockChangeAmtEl.value,
  });

  updateInventoryButtons(updatedInv);
}

function createInventoryButtons(inventoryArray) {
  const inventoryDiv = document.getElementById("sextantWrapper");
  selectedSextants = [];

  while (inventoryDiv.firstChild) {
    inventoryDiv.removeChild(inventoryDiv.firstChild);
  }

  inventoryArray.forEach((sextant) => {
    const sextantJson = JSON.parse(sextant);
    const sextantDiv = document.createElement("div");
    sextantDiv.classList.add("sextantButton");
    sextantDiv.id = sextantJson.name;

    var img = document.createElement("img");
    img.classList.add("sextant-icon");
    img.src = "/assets/" + sextantJson.icon;
    sextantDiv.append(img);

    const sextantName = document.createElement("h3");
    sextantName.textContent = sextantJson.name;
    sextantName.classList.add("sextant-name");
    sextantDiv.append(sextantName);

    const sextantTextDiv = document.createElement("div");
    sextantTextDiv.classList.add("sextantText");
    const sextantStock = document.createElement("h4");
    sextantStock.id = "stock";
    sextantStock.classList.add("stock-amt");
    sextantStock.textContent = sextantJson.stock;
    sextantTextDiv.append(sextantStock);
    const sextantPrice = document.createElement("h4");
    sextantPrice.textContent = sextantJson.price + "c";
    sextantTextDiv.append(sextantPrice);
    sextantDiv.append(sextantTextDiv);

    sextantDiv.addEventListener("click", function () {
      selectSextant(sextantJson.name);
    });

    inventoryDiv.append(sextantDiv);
  });
}

function updateInventoryButtons(inventoryArray) {
  inventoryArray.forEach((sextant) => {
    const sextantJson = JSON.parse(sextant);
    document
      .getElementById(sextantJson.name)
      .querySelector(".sextantText #stock").textContent = sextantJson.stock;
  });
}

function selectSextant(sextantName) {
  const index = selectedSextants.indexOf(sextantName);
  var sextantDiv = document.getElementById(sextantName);
  if (index > -1) {
    selectedSextants.splice(index, 1);
    sextantDiv.style.backgroundColor = "rgba(0, 0, 0, 0.466)";
  } else {
    selectedSextants.push(sextantName);
    sextantDiv.style.backgroundColor = "rgba(150, 150, 0, 0.3)";
  }
  console.log(selectedSextants);
}

function deselectAll() {
  selectedSextants.forEach((sextant) => {
    var sextantDiv = document.getElementById(sextant);
    sextantDiv.style.backgroundColor = "rgba(0, 0, 0, 0.466)";
  });

  document.getElementById("stock-change-amt").value = "";
  selectedSextants = [];
}

window.addEventListener("DOMContentLoaded", () => {
  inputTextEl = document.querySelector("#input-text");
  inventoryTextEl = document.querySelector("#inventory");
  stockChangeNameEl = document.querySelector("#stock-change-name");
  stockChangeAmtEl = document.querySelector("#stock-change-amt");
  document.querySelector("#load-txt-button").addEventListener("click", (e) => {
    e.preventDefault();
    getInventoryFromTxt();
  });
  document.querySelector("#stock-change-btn").addEventListener("click", (e) => {
    e.preventDefault();
    updateStock();
  });
  document.querySelector("#deselect-btn").addEventListener("click", (e) => {
    e.preventDefault();
    deselectAll();
  });
  getInventoryFromTxt();
});
