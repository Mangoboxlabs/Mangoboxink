# MongoBox-Protocol-ink
## Contract introduction
- MBTokenStore: Manages token minting and burning for all projects.
- MBFundingCycleStore:Manages funding cycle configurations and scheduling.
- MBProjects:Manages and tracks ownership over projects, which are represented as ERC-721 tokens.The protocol uses this to enforce permissions needed to access several project-oriented transactions.
- MBSplitsStore:Stores information about how arbitrary distributions should be split. The surface contracts currently use these to split up payout distributions and reserved token distributions.
- MBPrices:Manages and normalizes price feeds of various currencies.
- MBOperatorStore:Stores operator permissions for all addresses. Addresses can give permissions to any other address to take specific indexed actions on their behalf, while confining the permissions to an arbitrary number of domain namespaces.
- MBDirectory:Keeps a reference of which terminal contracts each project is currently accepting funds through, and which controller contract is managing each project's tokens and funding cycles.
- MBController:Stitches together funding cycles and project tokens, allowing for restricted control, accounting, and token management.
- MBSingleTokenPaymentTerminalStore: Manages accounting data on behalf of payment terminals that manage balances of only one token type.
- MBERC20PaymentTerminal:Manages the inflows and outflows of an ERC-20 token.
- MBToken:An ERC-20 token that can be used by a project in the `MBTokenStore`.




## Installing
### Before you begin
```
sudo apt install build-essential
```

At a minimum, you need the following packages before you install Rust:

```
clang curl git make
```
### Install required packages and Rust
```
sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler
```

Download the rustup installation program and use it to install Rust by running the following command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Update your current shell to include Cargo by running the following command:
```
source $HOME/.cargo/env
```
Please make sure that you have these prerequisites installed on your computer:

```bash
rustup default stable
rustup update
rustup update nightly
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

Then you have to install ink! command line utility which will make setting up Substrate smart contract projects easier:

```bash
cargo install cargo-contract --vers 0.15.0 --force --locked
```

You also need the [binaryen](https://github.com/WebAssembly/binaryen) package installed on your computer which is used to optimize the WebAssembly bytecode of the contract, you can use npm to install it:

```bash
npm install -g binaryen
```

## Testing

First of all you need to clone the repository, run:

```bash
git clone https://github.com/Mangoboxlabs/Mangoboxink.git
cd Mangoboxink/contract
```

Then, You can enter any folder and enter the following command.

```bash
cargo +nightly test
```

## Building

To build the WASM of your contract and metadata, You can enter any folder and enter the following command.
```bash
cargo +nightly contract build
```


