# BlackBox (Alpha)

BlackBox is a cross-platform tool for storing PGP keys securely and encrypting/decrypting messages. It is designed to provide a user-friendly experience with robust cryptographic capabilities, ensuring the security and privacy of sensitive communications.  

## Features
- Store and manage **PGP keys** (private/public)
- Encrypt and decrypt messages easily
- Cross-platform support (Windows, macOS, Linux, Android, iOS)
- Secure key storage using local encryption
- Simple and clean user interface

## Installation
**Coming Soon** – Platform-specific instructions will be available once the app moves out of alpha. 

As of right now just clone the repo and run `pnpm install` then run the dev command in package.json

## Development TODO
- Create Error type for project
- Make all db functions seperated into folder
- encrypting messages


## Potential Ideas
- Monero Wallet Integration


## Usage
1. **Add a PGP Key**  
   - Import or generate PGP keys inside the app.
2. **Encrypt a Message**  
   - Select a public key to encrypt the message.
3. **Decrypt a Message**  
   - Use the private key to decrypt incoming encrypted messages.

## Requirements
- [ ] Minimum OS version for each platform?  
- [ ] Any dependencies or libraries that need to be installed beforehand?  

## Security
- **How keys are stored**: Encrypted SQLite Database SQLCipher, Password protected
- **Does the app send or receive data remotely?** No

## Feedback and Contributions
We encourage testers and developers to report bugs and suggest features during the alpha phase!  
- **Issues and Feedback**: Open an issue on the GitHub page.  
- **Contributions**: See the [CONTRIBUTING.md](./CONTRIBUTING.md) file for contribution guidelines.  

## License
GPL V3


