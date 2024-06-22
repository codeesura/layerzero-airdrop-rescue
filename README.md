# LayerZero Airdrop Rescue Tool for Arbitrum

## 🚨 Emergency Wallet Recovery 🚨

This tool is designed to rescue compromised wallets eligible for the LayerZero airdrop on the Arbitrum network. If your wallet has been hacked and you're eligible for the LayerZero airdrop, this project might be your last line of defense.

## 📘 Table of Contents

- [Overview](#overview)
- [How It Works](#how-it-works)
- [Prerequisites](#prerequisites)
- [Setup and Usage](#setup-and-usage)
- [Environment Variables](#environment-variables)
- [Security Considerations](#security-considerations)
- [Disclaimer](#disclaimer)
- [Contact](#contact)

## Overview

The LayerZero Airdrop Rescue Tool is a specialized Rust-based solution designed to recover LayerZero (ZRO) tokens from compromised wallets on the Arbitrum network. It leverages quick transaction execution and proof verification to claim airdrops before unauthorized access can occur.

## How It Works

1. **Eligibility Verification**: Checks if the compromised wallet is eligible for the LayerZero airdrop.
2. **Proof Generation**: Creates a cryptographic proof required for the airdrop claim.
3. **Rapid Claim Execution**: Spams the Arbitrum network with claim transactions to ensure quick inclusion in a block.
4. **Safe Transfer**: Directly transfers claimed tokens to a secure wallet address.

## Prerequisites

- Rust 1.59 or higher
- Cargo (Rust's package manager)
- Access to an Arbitrum node (e.g., via Infura or Alchemy)
- LayerZero airdrop eligibility proof for the compromised wallet

## Setup and Usage

1. Install Rust and Cargo if you haven't already:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```
   git clone https://github.com/yourusername/layerzero-airdrop-rescue.git
   ```

3. Navigate to the project directory:
   ```
   cd layerzero-airdrop-rescue
   ```

4. Create a `.env` file and configure the environment variables (see [Environment Variables](#environment-variables) section).

5. Build the project:
   ```
   cargo build --release
   ```

6. Run the tool:
   ```
   cargo run --release
   ```

7. Follow the prompts to input the compromised wallet address and other required information.

## Environment Variables

Create a `.env` file in the root directory with the following variables:

```
RPC_URL=https://arbitrum.llamarpc.com
PRIVATE_KEY_FUNDING=your_funding_wallet_private_key
PRIVATE_KEY_HACKED=compromised_wallet_private_key
```

Make sure to replace the placeholder values:

- `PRIVATE_KEY_FUNDING`: Replace with the private key of the wallet you'll use to fund the rescue operation (for gas fees).
- `PRIVATE_KEY_HACKED`: Replace with the private key of the compromised wallet you're trying to rescue.

⚠️ **IMPORTANT SECURITY NOTE**: 
- Never share your private keys or commit them to version control.
- Ensure your `.env` file is included in your `.gitignore` to prevent accidental commits.
- After successful recovery, immediately transfer any remaining assets from the compromised wallet and discontinue use of that private key.

The `RPC_URL` is pre-configured to use a public Arbitrum RPC endpoint. If you prefer to use a different RPC provider (e.g., your own node or a service like Infura or Alchemy), you can change this URL.

## Security Considerations

- Use this tool only for wallets you own or have explicit permission to recover.
- Ensure the safe wallet address is correctly set to avoid sending tokens to the wrong destination.
- Keep your recovery session private; do not share screens or information while using this tool.
- After successful recovery, immediately transfer any other assets from the compromised wallet.
- Securely store and handle the private keys. Never share them or commit them to version control.

## Disclaimer

This tool is for emergency use in legitimate wallet recovery scenarios. It is not intended for, and should not be used for, any unauthorized access to wallets or funds. The user assumes all responsibility for the use of this tool.

## Contact

I specialize in wallet security and recovery operations for various blockchain networks. If you believe your wallet has been compromised or you need assistance with wallet security:

- Twitter: [@codeesura](https://twitter.com/codeesura) Email: codeesura@gmail.com

For urgent matters, please reach out via Twitter DM for the fastest response.

For general inquiries or to report issues with this tool, please open an issue in this repository.

---

Remember: In the world of blockchain, time is often critical when dealing with security issues. If you suspect your wallet has been compromised, act quickly but cautiously. Always prioritize the security of your private keys and be wary of sharing sensitive information.