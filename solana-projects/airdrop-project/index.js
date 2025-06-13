// Import required modules from the Solana Web3 library
const {
    Connection,       // allows us to connect to a Solana cluster (e.g., devnet, mainnet)
    PublicKey,        // represents a public key on Solana
    clusterApiUrl,    // gives URLs of different Solana clusters (like devnet)
    Keypair,          // helps create a new wallet keypair (public + private)
    LAMPORTS_PER_SOL  // conversion constant: 1 SOL = 1,000,000,000 lamports
} = require('@solana/web3.js');

// create a new random keypair (wallet) on Solana
const wallet = new Keypair(); 

// Access the public key from the keypair
const publicKey = new PublicKey(wallet._keypair.publicKey);

// Access the private key (secret key) from the keypair
const privateKey = wallet._keypair.secretKey;

console.log(publicKey);

console.log(privateKey);
