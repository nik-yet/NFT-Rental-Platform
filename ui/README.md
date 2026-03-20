# NFT Rental Platform UI

A lightweight permissionless NFT rental frontend for the Soroban contract at `contracts/hello-world`.

## Run local UI

1. `cd NFT-Rental-Platform/ui`
2. `python -m http.server 8000` (or any static host)
3. Open `http://localhost:8000`

## Notes

- `contractId` must be set after deploying the backend contract.
- This UI currently shows operation hooks and logging; the actual `SorobanClient` contract call code must be implemented in `app.js` for your project keys.
- The design is permissionless-first and no admin endpoint is required.

## Next integration step

- Add Soroban transaction building and submission using:
  - `SorobanClient.Server`, `SorobanClient.Keypair`, `SorobanClient.TransactionBuilder`
  - `SorobanClient.TransactionBuilder` method `.addOperation(sorobanOp)` for contract calls.
- Implement end-to-end wallet sign (using `soroban-wallet-js` or injected wallet plugin).
