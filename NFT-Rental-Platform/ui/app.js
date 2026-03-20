import { SorobanClient } from "https://unpkg.com/@stellar/soroban-client@0.31.0/dist/soroban-client.js";

const contractIdInput = document.getElementById("contractId");
const networkInput = document.getElementById("network");
const output = document.getElementById("output");

let server = null;
let contractId = null;

function log(msg) {
  output.textContent += `[${new Date().toISOString()}] ${msg}\n`;
  output.scrollTop = output.scrollHeight;
}

function connect() {
  const href = networkInput.value;
  contractId = contractIdInput.value.trim();
  if (!contractId || contractId === "YOUR_CONTRACT_ID") {
    alert("Set a valid contract ID first.");
    return;
  }
  server = new SorobanClient.Server(href);
  log(`Connected to ${href} contract ${contractId}`);
}

function getState() {
  const tokenId = Number(document.getElementById("tokenId").value);
  log(`Query state for token ${tokenId}`);
  // TODO: query using contract SDK call for get_owner, get_listing, get_rental.
  log(`Owner: [implementation pending]`);
  log(`Listing: [implementation pending]`);
  log(`Rental: [implementation pending]`);
}

function performAction(name) {
  const tokenId = Number(document.getElementById("tokenId").value);
  const owner = document.getElementById("owner").value.trim();
  const renter = document.getElementById("renter").value.trim();
  const price = Number(document.getElementById("price").value);
  const duration = Number(document.getElementById("duration").value);

  if (!server || !contractId) {
    alert("Connect first.");
    return;
  }

  log(`Action: ${name} (token=${tokenId})`);

  // This sample currently only logs. Replace with Soroban in-contract invocation.
  if (name === "mint") {
    log(`Mint ${tokenId} to ${owner}`);
  } else if (name === "list") {
    log(`List ${tokenId} for rent by ${owner} @ ${price}/sec max ${duration}s`);
  } else if (name === "rent") {
    log(`Rent ${tokenId} with renter ${renter} for ${duration}s`);
  } else if (name === "end") {
    log(`End rental of ${tokenId}`);
  } else if (name === "transfer") {
    log(`Transfer ${tokenId} from ${owner} to ${renter}`);
  }

  log("(Note: Implement contract transaction building and submit with keypair.)");
}

document.getElementById("connectBtn").addEventListener("click", connect);
document.getElementById("refreshBtn").addEventListener("click", getState);
document.getElementById("mintBtn").addEventListener("click", () => performAction("mint"));
document.getElementById("listBtn").addEventListener("click", () => performAction("list"));
document.getElementById("rentBtn").addEventListener("click", () => performAction("rent"));
document.getElementById("endBtn").addEventListener("click", () => performAction("end"));
document.getElementById("transferBtn").addEventListener("click", () => performAction("transfer"));
