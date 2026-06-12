# ORGVault 

## Project Overview

ORGVault is a cross platform desktop application designed to provide organizations with a highly secure, encrypted solution for storing, sharing, and synchronizing sensitive documents across multiple devices within a local area network (LAN). The solution ensures confidentiality, controlled access, offline usability, and comprehensive audit logging while defending against ransomware attacks through immutable backups and version control.

---

## Key Features

- **Cross-Platform Support:** Built with Rust, server is made for Linux, Client can run on Windows, MacOS & Linux.  
- **Strong Encryption:** All stored documents are encrypted using industry standard AES-256. SHA-256 hashes are used for file integrity checks.
- **Two-Way Synchronization:** Seamless encrypted synchronization of files between local desktop clients and a central backend server over LAN.
- **Offline Functionality:** Full access to stored documents is available when offline. Changes are auto-synced once connectivity is restored.
- **Role-Based and Group-Based Access Control:** Fine grained permission settings for sharing documents securely among different users and groups.
- **Audit Trails:** All file accesses, modifications, sharing actions, and synchronization events are logged securely and viewable in-browser.
- **Ransomware Protection:** Incorporates version-controlled document backup and immutable storage to defend against tampering or deletion.
- **Lightweight & Easy Deployment:** The application is lightweight and designed for simple deployment in organizational intranets using Rust tooling.
- **User authentication:** It is done with the help of MAC address for safety & reliability. 

---

## Architecture Overview

- **UI:** Based on `next.js` with `tailwindcss` for design ran with the help of rust.
- **Async Runtime:** Tokio runtime (integrated within Actix Web) handles all asynchronous operations.
- **API Layer:** Actix Web's backend api methods will be called to communicate with frontend. 
- **Data Storage:** Encrypted JSON document manages metadata, audit logs, and file encryption keys alongside document storage.

---

## Security Considerations

- **Encryption:** Uses AES-256 for all file content encryption. Keys are securely managed and never stored in plaintext.
- **Data Integrity:** SHA-256 hashes verify file integrity during sync.
- **Access Control:** Strict role-based access ensures only authorized users can view or modify documents.
- **Ransomware Defense:** Versioning and immutable backups guard against malicious data loss.
- **Network Security:** Sync communications are limited to LAN and protected by MAC address authentication to prevent unauthorized access.

---

## Setup
### Prerequisites
- Rust
- Cargo
- Yarn
- Node.js

### Installation
1. Clone the repository
2. Navigate to the project directory
3. Run `cargo run`