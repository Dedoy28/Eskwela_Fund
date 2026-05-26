# EskwelaFund
An automated on-chain disbursement system for public school student allowances and NGO grants.

## Problem & Solution
**Problem:** Students in underfunded public schools rely on delayed, physical cash disbursements from LGUs and NGOs, forcing them to skip meals or face transportation hurdles when funds are delayed.
**Solution:** EskwelaFund uses a Soroban smart contract to automatically disburse USDC directly to student wallets based on school admin authorization, ensuring instant, transparent, and auditable fund delivery.

## Timeline
Can be successfully demoed and tested end-to-end within a standard 48-hour hackathon timeframe.

## Stellar Features Used
* USDC (Stellar Asset) transfers
* Soroban smart contracts (Storage, Auth, Token Invocation)

## Vision and Purpose
To completely eradicate the logistical nightmare and lack of transparency in local educational fund distribution, bringing real financial inclusion to unbanked youth through stablecoin technology.

## Prerequisites
* Rust toolchain (`rustup target add wasm32-unknown-unknown`)
* Soroban CLI (`v25.0.0` or latest compatible matching the workspace dependencies)

## How to build
```bash
soroban contract build


<img width="1920" height="943" alt="Screenshot 2026-05-26 121247" src="https://github.com/user-attachments/assets/9c5c6fee-88e3-4969-b2f6-30716e2adaea" />


