const express = require('express');
const { ProxyNetworkProvider } = require('@multiversx/sdk-network-providers');
const { Address, TransactionPayload, Transaction } = require('@multiversx/sdk-core');
const { TransactionBuilder } = require('@multiversx/sdk-transaction-builder');
const { Wallet } = require('@multiversx/sdk-wallet');
const axios = require('axios');

const app = express();
const port = 3000;

app.use(express.json()); // Enable parsing JSON request bodies

// Configuration (replace with your values)
const proxy = new ProxyNetworkProvider("https://testnet-gateway.multiversx.com"); // Testnet Gateway
const contractAddress = "erd1..."; // Your contract address
const chainId = "T"; // Testnet Chain ID

// Function to build and send a transaction
async function sendTransaction(senderPrivateKey, recipient, amount, data) {
  const senderWallet = Wallet.fromPrivateKey(senderPrivateKey);
  const senderAddress = senderWallet.getAddress();

  const transaction = new Transaction({
    nonce: await proxy.getAccount(senderAddress).then((account) => account.nonce),
    sender: senderAddress,
    receiver: new Address(contractAddress),
    value: amount, // Amount in WEGLD
    gasLimit: 6000000, // Adjust as needed
    gasPrice: 1000000000,
    data: new TransactionPayload(data),
    chainID: chainId,
  });

  const transactionBuilder = new TransactionBuilder({ transaction });

  const signedTransaction = transactionBuilder.buildSigned(senderWallet);

  const txHash = await proxy.sendTransaction(signedTransaction);
  return txHash.toString();
}

// Endpoint to create an offer
app.post('/createOffer', async (req, res) => {
  try {
    const { senderPrivateKey, recipient, amount } = req.body;
    const data = `create@${new Address(recipient).toString()}`; // Function call data
    const txHash = await sendTransaction(senderPrivateKey, recipient, amount, data);
    res.json({ txHash });
  } catch (error) {
    console.error("Error creating offer:", error);
    res.status(500).json({ error: error.message });
  }
});

// Endpoint to get all offers for a user
app.get('/getOffers/:userAddress', async (req, res) => {
  try {
    const userAddress = req.params.userAddress;
    const offers = await proxy.queryContract({
      contract: contractAddress,
      func: "getUserAllOffers",
      args: [new Address(userAddress).toHex()],
    });
    res.json(offers);
  } catch (error) {
    console.error("Error getting offers:", error);
    res.status(500).json({ error: error.message });
  }
});

// Endpoint to cancel an offer
app.post('/cancelOffer', async (req, res) => {
  try {
    const { senderPrivateKey, offerId } = req.body;
    const data = `cancelOffer@${offerId}`; // Function call data
    const txHash = await sendTransaction(senderPrivateKey, contractAddress, 0, data);
    res.json({ txHash });
  } catch (error) {
    console.error("Error canceling offer:", error);
    res.status(500).json({ error: error.message });
  }
});

// Endpoint to accept an offer
app.post('/acceptOffer', async (req, res) => {
  try {
    const { senderPrivateKey, offerId } = req.body;
    const data = `acceptOffer@${offerId}`; // Function call data
    const txHash = await sendTransaction(senderPrivateKey, contractAddress, 0, data);
    res.json({ txHash });
  } catch (error) {
    console.error("Error accepting offer:", error);
    res.status(500).json({ error: error.message });
  }
});

// Implement additional endpoints as needed (e.g., get specific offer details)

app.listen(port, () => {
  console.log(`Server listening at http://localhost:${port}`);
});