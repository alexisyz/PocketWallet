# **Pocket Wallet 🛡️**

**Pocket Wallet: A secure and portable 256-bit seed phrase generator.**

1. Compiled to WebAssembly (WASM), allowing it to run on any device (Mobile, Tablet, Desktop) via a web browser without internet dependency.

2. All calculations are performed locally on your device. Your password and salt never leave your machine.

### Technical Stack

    Language: Rust 🦀

    Standard: BIP-39 (Mnemonic phrases)

    Hashing: Argon2id (Key Derivation Function)

    Deployment: GitHub Pages + WebAssembly
### 🌐 Live Demo
You can use the tool directly here: [https://alexisyz.github.io/PocketWallet/](https://alexisyz.github.io/PocketWallet/)
#

# 🚀 How to Run Pocket Wallet in your PC

Pocket Wallet is a Command Line Interface (CLI) tool. It does not have a graphical window; instead, it runs directly inside your terminal or console to ensure maximum security and performance.

## 🐧 Linux (Ubuntu, Arch, Fedora, etc.)

**Since the binary is a standalone file, you need to grant it execution permissions before running it.**
1) Open your terminal.
   Navigate to the folder where you downloaded the pocket-wallet file:

       cd /path/to/your/download

2) Grant execution permissions:

        chmod +x pocket-wallet

3) Run the application:

        ./pocket-wallet
## 🪟 Windows

**On Windows, the process is similar. Avoid double-clicking the file, as the window will close immediately after finishing.**
1) Open PowerShell or Command Prompt (CMD). Navigate to the folder containing the executable:

        cd C:\Users\YourName\Downloads

2) Run the application:

        .\pocket-wallet.exe

*By Navi Collective*
