# Create an ICRC2 Token at Lightning Speed

Easily deploy and create an ICRC2 token on the Internet Computer with these simple steps.

## Instructions

1. **Clone the Repository**

2. **Set Executable Permissions**

   After cloning the repository and ensuring you have DFX installed, run the following commands to set the correct permissions for the shell scripts:
   ```bash

   chmod +x download_ledger.sh
   chmod +x token.sh
   ```
   To deploy run ```npm run deploy```

   It deploys to the local network

   To deploy to mainnet , edit token.sh file here, ```dfx deploy icrc1_ledger_canister ``` to include ```--network ic``` flag and ensure you have cycles by redeemig a coupun or converting icp tokens to cycles.