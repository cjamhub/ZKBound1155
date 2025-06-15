# Build Project

## Setup Environment
``` bash
# support hardhat
npm install --save-dev hardhat
npx hardhat init
# support openzeppelin
npm install @openzeppelin/contracts

# support foundry within a hardhat project
npm install --save-dev @nomicfoundation/hardhat-foundry
add `import "@nomicfoundation/hardhat-foundry";` to hardhat.config.js
npx hardhat init-foundry
forge install --no-commit foundry-rs/forge-std
# foundry.toml add remappings
# ---

# support plonky2
rustup override set nightly
cargo init
add `jemallocator = "0.5.0"` to Cargo.toml
```
