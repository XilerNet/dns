# Rust Decentralized DNS Node (Alpha)

Welcome to the Xiler Decentralized DNS Node (Xiler DDNS)! This is an alpha version of a decentralized DNS node written in Rust, designed to enable a peer-to-peer DNS system on Bitcoin Ordinals Theory. In this README, we'll provide an overview of the project, its current status, and the upcoming features and improvements.

## Table of Contents

* [Overview](#overview)
* [Features](#features)
* [Getting](#started-getting)
* [Upcoming](#features-upcoming)
* [Contributing](#contributing)
* [License](#license)

## Overview

Xiler DDNS is a project aimed at creating a decentralized, peer-to-peer DNS system using Rust. The decentralized nature of this project allows for censorship-resistant and more robust DNS services. We believe in the power of decentralization to enhance the resilience and security of the internet.

**Current Status**: `Alpha`

This means that while Xiler DDNS is already functional, it's not yet production-ready, and there's room for improvements and additional features.

## Features

As of this alpha version, Xiler DDNS already includes:

* **Name Resolution**: The system can resolve DNS queries within the network, enabling DNS functionality.

* **Partial recursive**: Will look up the off-chain dns records if present *(eg CNAME .o to a .com will resolve within the DNS node)*

* **IPv4 & IPv6 Support**: Currently, Xiler DDNS supports IPv4 and IPv6 addresses.

* **A, NS, CNAME, MX, AAAA records**: Supports all these records right now

* **Crypographical signature support**: Supports ed25519, dilithium2, dilithium2aes, dilithium3, dilithium3aes, dilithium5, dilithium5aes as signature algorithm

## Getting Started

To get started, just run the seaorm migrations and run `cargo run --bin xdns` and `cd inscription-indexer && cargo run`.

## Upcoming Features

In the upcoming releases, we plan to implement several critical features to improve the system's functionality and security. Here are the key features on our roadmap:

### 1. Drop Operator

The "drop" operator will be implemented to allow DNS records and domains to be deleted. Will require a validity to be set.

### 2. DNSSEC Support

Implementing DNSSEC (Domain Name System Security Extensions) is a crucial step to enhance the security of our decentralized DNS network. This feature will enable the validation of DNS responses and allow public nodes to join the network securely.

#### 3. DNS records chain check

Check if the dns record its provided chain inscription id is valid. If not ignore record.

## Contributing

We welcome contributions from the open-source community to help us improve Xiler DDNS.

## License

Xiler DDNS is open-source software and is released under the [MIT License](LICENSE). Please review the license file for more details.

Thank you for exploring Xiler DDNS! We look forward to your contributions and hope to grow this project into a powerful, decentralized DNS solution.
