const express = require('express');
const path = require('path');
const { ProxyNetworkProvider } = require('@multiversx/sdk-network-providers');
const { Address, TransactionPayload, Transaction } = require('@multiversx/sdk-core');
const { UserSigner } = require('@multiversx/sdk-wallet');

const app = express();
const port = 4000;

// Middleware
app.use(express.json());
app.use(express.static(path.join(__dirname)));

// Configuration
const proxy = new ProxyNetworkProvider("https://devnet-gateway.multiversx.com");
const contractAddress = "erd1qlp7560vds526xgkl88d4s0atq67yts6l86vgrv3m4w5xdu8rprssdglu0";
const chainId = "D";

async function sendTransaction(senderPrivateKey, value, data) {
    try {
        // Convert hex private key to buffer and create signer
        const secretKey = Buffer.from(senderPrivateKey, 'hex');
        const signer = new UserSigner(secretKey);
        
        // Get the address
        const senderAddress = signer.getAddress();
        console.log('Address:', senderAddress.bech32());

        const transaction = new Transaction({
            nonce: await proxy.getAccount(senderAddress).then((account) => account.nonce),
            value: value,
            sender: senderAddress,
            receiver: new Address(contractAddress),
            gasLimit: 60000000,
            gasPrice: 1000000000,
            data: new TransactionPayload(data),
            chainID: chainId,
            version: 1
        });

        const signature = await signer.sign(transaction.serializeForSigning());
        transaction.applySignature(signature);

        const txHash = await proxy.sendTransaction(transaction);
        return txHash.toString();
    } catch (error) {
        console.error('Transaction error:', error);
        throw error;
    }
}

// Routes
app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'index.html'));
});

// Create offer endpoint
app.post('/createOffer', async (req, res) => {
    try {
        const { senderPrivateKey, recipient, amount } = req.body;
        const data = `create@${new Address(recipient).hex()}`;
        const txHash = await sendTransaction(senderPrivateKey, amount, data);
        res.json({ txHash });
    } catch (error) {
        console.error("Error creating offer:", error);
        res.status(500).json({ error: error.message });
    }
});

// Get offers endpoint
app.get('/getOffers/:userAddress', async (req, res) => {
    try {
        const userAddress = req.params.userAddress;
        const query = {
            scAddress: contractAddress,
            funcName: "getUserActiveOffers",
            args: [new Address(userAddress).hex()]
        };
        const offers = await proxy.queryContract(query);
        res.json(offers);
    } catch (error) {
        console.error("Error getting offers:", error);
        res.status(500).json({ error: error.message });
    }
});

// Add endpoint to get incoming offers
app.get('/getIncomingOffers/:userAddress', async (req, res) => {
    try {
        const userAddress = req.params.userAddress;
        const query = {
            scAddress: contractAddress,
            funcName: "getUserIncomingActiveOffers",
            args: [new Address(userAddress).hex()]
        };
        const offers = await proxy.queryContract(query);
        res.json(offers);
    } catch (error) {
        console.error("Error getting incoming offers:", error);
        res.status(500).json({ error: error.message });
    }
});

// Cancel offer endpoint
app.post('/cancelOffer', async (req, res) => {
    try {
        const { senderPrivateKey, offerId } = req.body;
        const data = `cancelOffer@${offerId}`;
        const txHash = await sendTransaction(senderPrivateKey, "0", data);
        res.json({ txHash });
    } catch (error) {
        console.error("Error canceling offer:", error);
        res.status(500).json({ error: error.message });
    }
});

// Accept offer endpoint
app.post('/acceptOffer', async (req, res) => {
    try {
        const { senderPrivateKey, offerId } = req.body;
        const data = `acceptOffer@${offerId}`;
        const txHash = await sendTransaction(senderPrivateKey, "0", data);
        res.json({ txHash });
    } catch (error) {
        console.error("Error accepting offer:", error);
        res.status(500).json({ error: error.message });
    }
});

// Add endpoint to get all active offers
app.get('/getActiveOffers', async (req, res) => {
    try {
        const query = {
            scAddress: contractAddress,
            funcName: "getActiveOffers",
            args: []
        };
        const offers = await proxy.queryContract(query);
        res.json(offers);
    } catch (error) {
        console.error("Error getting active offers:", error);
        res.status(500).json({ error: error.message });
    }
});

// Start server
app.listen(port, () => {
    console.log(`Server running at http://localhost:${port}`);
    console.log('Available endpoints:');
    console.log('- GET  /');
    console.log('- POST /createOffer');
    console.log('- GET  /getOffers/:userAddress');
    console.log('- GET  /getIncomingOffers/:userAddress');
    console.log('- POST /cancelOffer');
    console.log('- POST /acceptOffer');
    console.log('- GET  /getActiveOffers');
});