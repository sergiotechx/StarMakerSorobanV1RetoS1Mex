# 🚀 Soroban Calculator Smart Contract

![Stellar](https://img.shields.io/badge/Stellar-Soroban-7D00FF)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange)
![License](https://img.shields.io/badge/License-Apache%202.0-blue)

A simple calculator smart contract built on Soroban, Stellar's smart contract platform. This project demonstrates basic functionality of Soroban contracts including computation and state persistence.

## 📋 Features

- Addition of two integers
- State persistence between contract invocations
- Comprehensive test suite

## 🛠️ Technical Implementation

This contract implements two main functions:

1. **`sumar`** - Adds two integers and stores the result in the contract's persistent storage
2. **`resultado_anterior`** - Retrieves the previously calculated sum from storage

The implementation leverages Soroban's storage capabilities to maintain state between function calls.

## 🔧 Getting Started

### Prerequisites

- Rust and Cargo
- Soroban CLI

### Building the Contract

```bash
# Build the contract
cargo build

# Build for Soroban
stellar contract build
```

### Running Tests

```bash
cargo test
```

## 📚 Code Structure

- `lib.rs` - Main contract implementation
- `test.rs` - Test suite for the contract

## 🧪 Testing

The contract includes a comprehensive test suite that verifies:

- Basic addition functionality
- Multiple consecutive operations
- Initial state handling
- State persistence between function calls

## 📝 Original Challenge Instructions

**Reto Workshop Soroban Sesión 1**

😉**Pistas y guias para el reto:** [https://developers.stellar.org/docs/build/smart-contracts](https://developers.stellar.org/docs/build/smart-contracts)

*   implementar la función _sumar_, se le envian 2 números de parámetros
*   implementar _resultado\_anterior_, este debe retornar el valor que dió la suma anteriormente (Storing Data), esto implica hacer algo en la función sumar🤔

_Hacer fork del respositorio y cuando esté resuelto, subir la solución a tu github_

_debe dar ok el test automatico_

```plaintext
cargo test
```

## 📖 Soroban Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Developer Documentation](https://developers.stellar.org/docs)
- [Soroban Examples](https://github.com/stellar/soroban-examples)

## 📄 License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.
