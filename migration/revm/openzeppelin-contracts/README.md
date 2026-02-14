# OpenZeppelin Contracts - REVM Testing

[![REVM OpenZeppelin Contracts](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/migration-revm-openzeppelin-contracts.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/migration-revm-openzeppelin-contracts.yml)

This project tests OpenZeppelin Contracts on Polkadot's REVM (Ethereum-compatible EVM).

## What This Tests

- ERC20, ERC721, ERC1155 token standards
- Access control contracts (Ownable, AccessControl)
- Proxy and upgradeable patterns
- Governance contracts
- Utility contracts (Address, Strings, Math, etc.)

## Prerequisites

- Node.js 22+

## Running Tests

```bash
npm ci
npm test
```

## Source

- [openzeppelin-contracts-revm](https://github.com/papermoonio/openzeppelin-contracts-revm)
- [Original OpenZeppelin Contracts](https://github.com/OpenZeppelin/openzeppelin-contracts)
