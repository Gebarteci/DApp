# MultiversX Escrow DApp

A decentralized escrow application built on the MultiversX blockchain that allows users to create, accept, and cancel escrow offers. The smart contract ensures secure transfers between parties by holding funds until the recipient accepts the offer.

## Overview

This project consists of:
- A Rust smart contract for handling escrow logic
- A Node.js Express backend server
- A simple HTML/JavaScript frontend interface

The smart contract is deployed on the MultiversX Devnet at:
`erd1qlp7560vds526xgkl88d4s0atq67yts6l86vgrv3m4w5xdu8rprssdglu0`

## Smart Contract Features

### Core Functions

1. **Create Offer**
   - Creates an escrow offer with specified recipient and amount
   - Requires EGLD payment
   - Emits a `createOffer` event

2. **Accept Offer**
   - Allows recipient to accept an active offer
   - Transfers EGLD to the creator
   - Includes reentrancy protection
   - Emits an `acceptOffer` event

3. **Cancel Offer**
   - Allows creator to cancel their active offer
   - Returns EGLD to the creator
   - Emits a `cancelOffer` event

### View Functions

- `getActiveOffers`: Returns all active escrow offers
- `getUserActiveOffers`: Returns active offers created by a specific user
- `getUserIncomingActiveOffers`: Returns active offers where user is the recipient

## Development Setup

### Prerequisites

- Rust (latest stable)
- Node.js (>=14.0.0)
- mxpy (MultiversX SDK)

### Smart Contract Deployment

1. Build the contract:

bash
cd contract
mxpy contract build (sc-meta all build)

2.Deploy the devnet (old method)

bash
mxpy contract deploy --bytecode=output/escrow-contract.wasm \
--recall-nonce --pem=<your-wallet.pem> \
--gas-limit=60000000 \
--proxy=https://devnet-gateway.multiversx.com \
--chain=D --send

### Running Tests

The smart contract includes comprehensive tests covering core functionality:

bash
cd contract
cargo test


### Running the UI

1. Install dependencies:
bash
cd ui
npm install

2. Start the server:
bash
npm start

The application will be available at `http://localhost:4000`

## User Interface

The DApp provides a simple interface for:
- Creating new escrow offers
- Viewing active offers
- Accepting incoming offers
- Canceling created offers

### API Endpoints

- `POST /createOffer`: Create new escrow offer
- `GET /getOffers/:userAddress`: Get user's active offers
- `GET /getIncomingOffers/:userAddress`: Get incoming offers
- `POST /cancelOffer`: Cancel an existing offer
- `POST /acceptOffer`: Accept an incoming offer
- `GET /getActiveOffers`: Get all active offers

## Security Features

- Reentrancy protection for accept_offer function
- Proper access control for offer operations
- Input validation and error handling
- Secure EGLD transfer handling

