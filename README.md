Here’s a README.md file for the yogachain-common repository:

YogaChain Common Library (yogachain-common) 🧘‍♂️🔗

The yogachain-common repository contains shared utilities, cryptographic functions, types, and logging tools used across the YogaChain ecosystem. This library serves as the foundation for multiple repositories, ensuring code reusability, security, and efficiency.

📌 Key Features

✅ Shared Data Types – Standardized structs and enums for blockchain transactions, networking, and consensus.
✅ Common Cryptographic Functions – Quantum-safe cryptographic utilities (Kyber, Dilithium, BLAKE3).
✅ Utility Functions – Helper methods for encoding, decoding, hashing, and signature verification.
✅ Logging Framework – Standardized logging for debugging and monitoring.
✅ Cross-Repository Compatibility – Used by Core, Networking, AI-Modules, Contracts, and XCVM-Executor.

📁 Repository Structure

yogachain-common
│── Cargo.toml               # Project metadata and dependencies
│── .gitignore               # Ignore unnecessary files
│── README.md                # Project documentation
│── src
│    │── lib.rs              # Entry point for common modules
│    │── types.rs            # Shared data types (structs, enums)
│    │── crypto.rs           # Cryptographic utilities (Kyber, Dilithium, BLAKE3)
│    │── utils.rs            # General-purpose helper functions
│    │── logging.rs          # Logging and debugging tools

📦 Installation

This repository is designed to be included as a dependency in other YogaChain modules.

Using in Your Rust Project
	1.	Add to Cargo.toml

[dependencies]
yogachain-common = { git = "https://github.com/YogaChain/yogachain-common.git" }

	2.	Import in Your Code

use yogachain_common::crypto::generate_keypair;
use yogachain_common::types::Transaction;
use yogachain_common::logging::init_logger;

🔑 Cryptography

The yogachain-common module implements quantum-resistant cryptographic functions, ensuring high security:
	•	Kyber – Quantum-safe key encapsulation.
	•	Dilithium – Secure digital signatures.
	•	BLAKE3 – High-speed cryptographic hashing.

Example: Digital Signature

use yogachain_common::crypto::{sign_message, verify_signature};

let message = b"YogaChain transaction";
let (public_key, private_key) = generate_keypair();
let signature = sign_message(message, &private_key);
assert!(verify_signature(message, &signature, &public_key));

📊 Logging & Debugging

Standardized logging is implemented using log and env_logger.

Example: Initialize Logger

use yogachain_common::logging::init_logger;

fn main() {
    init_logger();
    log::info!("YogaChain Common Library Initialized.");
}

🚀 Contributions

We welcome contributions! Please follow these steps:
	1.	Fork the repository 🍴
	2.	Create a feature branch 🛠 (git checkout -b feature-name)
	3.	Commit changes ✅ (git commit -m "Added new feature")
	4.	Push to GitHub 📤 (git push origin feature-name)
	5.	Open a pull request 🔥

📜 License

This project is licensed under the MIT License. See the LICENSE file for details.

🔗 Related Repositories
	•	YogaChain Core – Core blockchain logic.
	•	YogaChain Networking – P2P communication.
	•	YogaChain AI Modules – AI-powered enhancements.
	•	YogaChain Contracts – Smart contract execution.
	•	YogaChain XCVM Executor – Cross-chain virtual machine.

📧 Contact & Support

For any issues or contributions, please open a GitHub Issue or reach out to the YogaChain team at:
📩 Email: kavi4k@yogachain.net
🌍 Website: YogaChain.net

🚀 YogaChain – Building the Future of Decentralized, Quantum-Safe Blockchains!
