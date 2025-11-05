# Comprehensive Discrepancies: All Polkadot Docs Tutorials & Guides

**Date:** November 3, 2025
**Scope:** ALL 27 tutorials + ALL 75+ guides
**Source:** Deep analysis of Polkadot Docs repository

---

## Summary Statistics

| Category | Total | Easy | Medium | Hard | Not Applicable |
|----------|-------|------|--------|------|----------------|
| **Tutorials** | 27 | 8-10 (30-37%) | 8-10 (30-37%) | 6-8 (22-30%) | 4-5 (15-18%) |
| **Guides** | 75+ | - | 10-15 (13-20%) | - | 60-65 (80-87%) |

---

## Major Discrepancy Types

### 1. GUI Dependencies (37% of tutorials)

**Affected Tutorials (10+):**
- Deploy ERC-20, Deploy NFT - **Remix IDE** (browser-based)
- Para-to-Para Channels, Para-to-System Channels - **Polkadot.js Apps UI**
- Register Local Asset, Register Foreign Asset, Asset Conversion - **Polkadot.js Apps UI**
- Runtime Upgrade (submission step) - **Polkadot.js Apps UI**
- dApp tutorials - **MetaMask/Talisman** browser wallets

**Impact:** Cannot convert without converting UI interactions to code (PAPI)

---

### 2. External Services (55% of tutorials)

**Common Dependencies:**
- **Faucets:** PAS tokens, Westend/Paseo testnet tokens
- **Live Networks:** Relay chains + parachains running
- **Binaries:** polkadot, polkadot-parachain, substrate-node, eth-rpc-adapter, frame-omni-bencher, chain-spec-builder
- **Marketplaces:** RegionX for coretime

**Impact:** Recipes cannot be fully self-contained

---

### 3. Multi-Language Setups (18% of tutorials)

**Examples:**
- Runtime development (Rust) + deployment scripts (TypeScript)
- Pallet development (Rust) + frontend (React/TypeScript)
- Uniswap V2 (Solidity + Hardhat + node binaries)

**Impact:** CLI currently supports single-language recipes only

---

### 4. Frontend/dApp Development (7% of tutorials)

**Affected:**
- Create dApp with Ethers.js
- Create dApp with Viem

**Requirements:**
- Next.js framework
- React components
- Browser wallet integration
- Dev server workflows

**Impact:** Missing pathway - need new "Frontend/dApp" pathway

---

### 5. Infrastructure/Deployment Guides (15-18% of tutorials)

**Affected:**
- Para-to-Para HRMP Channels - Pure UI workflow, no code
- Para-to-System HRMP Channels - Infrastructure setup
- Deploy to Testnet - Multi-step operational procedure
- Obtain Coretime - Resource acquisition guide

**Impact:** These are guides, not executable tutorials - don't fit recipe model

---

## Tutorials Breakdown by Category

### DApps (1 tutorial)

| Tutorial | Pathway | Difficulty | Key Discrepancies |
|----------|---------|------------|-------------------|
| Remark Tutorial | basic-interaction | MEDIUM | Browser wallet, audio file, testnet dependency |

---

### Interoperability (5 tutorials)

| Tutorial | Pathway | Difficulty | Key Discrepancies |
|----------|---------|------------|-------------------|
| XCM Fee Estimation | xcm | EASY | None - works well ✅ |
| XCM Transfers (Relay→Para) | xcm | EASY | Minimal - pre-configured HRMP |
| Replay & Dry-Run XCMs | xcm | MEDIUM | Custom runtime compilation |
| Para-to-Para Channels | - | NOT APPLICABLE | **Pure UI workflow (Polkadot.js Apps)** |
| Para-to-System Channels | - | NOT APPLICABLE | **Pure UI workflow (Polkadot.js Apps)** |

---

### Onchain Governance (1 tutorial)

| Tutorial | Pathway | Difficulty | Key Discrepancies |
|----------|---------|------------|-------------------|
| Fast-Track Governance | basic-interaction | MEDIUM | Uses @polkadot/api (not PAPI), Chopsticks manipulation |

---

### Polkadot SDK - Zero to Hero (8 tutorials)

| Tutorial | Pathway | Difficulty | Key Discrepancies |
|----------|---------|------------|-------------------|
| Build Custom Pallet | runtime | EASY | None - perfect fit ✅ |
| Pallet Unit Testing | runtime | EASY | None - perfect fit ✅ |
| Add Pallets to Runtime | runtime | EASY | None - perfect fit ✅ |
| Set Up Template | runtime | MEDIUM | Binary installation (omni-node, chain-spec-builder) |
| Pallet Benchmarking | runtime | MEDIUM | frame-omni-bencher tool, Handlebars templates |
| Runtime Upgrade | runtime | MEDIUM | Polkadot.js Apps for submission |
| Deploy to Testnet | - | NOT APPLICABLE | **Multi-step deployment workflow, not code** |
| Obtain Coretime | - | NOT APPLICABLE | **Marketplace interaction, testnet-specific** |

---

### System Chains - Asset Hub (4 tutorials)

| Tutorial | Pathway | Difficulty | Key Discrepancies |
|----------|---------|------------|-------------------|
| Pay Fees Different Token | basic-interaction | EASY | None - works well ✅ |
| Asset Conversion | basic-interaction | MEDIUM | **Polkadot.js Apps UI** - needs PAPI conversion |
| Register Foreign Asset | xcm | HARD | **Multi-chain + Polkadot.js Apps UI** |
| Register Local Asset | basic-interaction | HARD | **7-step UI workflow** - needs PAPI code |

---

### Testing (2 tutorials)

| Tutorial | Pathway | Difficulty | Key Discrepancies |
|----------|---------|------------|-------------------|
| Fork Live Chains (Chopsticks) | testing | EASY | None - works well ✅ |
| Spawn Basic Chain (Zombienet) | testing | MEDIUM | Multiple binary downloads, TOML config |

---

### Smart Contracts (6 tutorials)

| Tutorial | Pathway | Difficulty | Key Discrepancies |
|----------|---------|------------|-------------------|
| Create Smart Contract | contracts | EASY | Just contract code - part of larger recipe |
| Test & Deploy Hardhat | contracts | MEDIUM | **substrate-node + eth-rpc-adapter binaries** |
| Deploy ERC-20 | contracts | HARD | **Remix IDE** (web-based) - convert to Hardhat |
| Deploy NFT | contracts | HARD | **Remix IDE** (web-based) - convert to Hardhat |
| Deploying Uniswap V2 | contracts | HARD | **Multi-binary setup**, large codebase |
| Create dApp (Ethers.js) | **Frontend** ❌ | HARD | **Next.js + browser wallet** - new pathway needed |
| Create dApp (Viem) | **Frontend** ❌ | HARD | **Next.js + browser wallet** - new pathway needed |

---

## Guides Analysis

### Total: 75+ guides

**Breakdown by Type:**

1. **Reference Documentation (60% - 45+ guides)**
   - Examples: `intro-to-xcm.md`, `xcm-config.md`, `smart-contracts/overview.md`
   - **Don't Convert** - These are reference docs, not executable tutorials

2. **Setup Guides (20% - 15 guides)**
   - Examples: `install-polkadot-sdk.md`, `dev-environments/hardhat.md`
   - **Some Convertible** - 5-10 could become recipes

3. **How-To Guides (15% - 11 guides)**
   - Examples: `add-existing-pallets.md`, `libraries/ethers-js.md`
   - **Convertible** - Good recipe candidates

4. **Overview/Info (5% - 4 guides)**
   - Examples: FAQs, network information
   - **Don't Convert** - Conceptual content

---

## Discrepancy Patterns

### Pattern 1: GUI-First Tutorials

**Characteristics:**
- Use Polkadot.js Apps or Remix IDE for all operations
- Little to no code provided
- Focus on UI navigation

**Conversion Strategy:**
- Rewrite using PAPI (for Polkadot.js Apps)
- Rewrite using Hardhat (for Remix IDE)

**Effort:** 2-4 hours per tutorial

---

### Pattern 2: Binary-Heavy Tutorials

**Characteristics:**
- Require multiple external tools
- Complex PATH configuration
- Version-specific dependencies

**Conversion Strategy:**
- Add dependency management to CLI
- Provide installation automation
- Docker containers as alternative

**Effort:** CLI enhancement needed first

---

### Pattern 3: Multi-Tool Orchestration

**Characteristics:**
- Chopsticks + PAPI
- Zombienet + Polkadot.js API
- Hardhat + substrate-node + eth-rpc-adapter

**Conversion Strategy:**
- Add multi-tool orchestration to CLI
- Provide startup scripts
- Clear documentation on dependencies

**Effort:** 1-3 hours per tutorial (after CLI work)

---

### Pattern 4: Infrastructure/Deployment

**Characteristics:**
- No executable code
- Multi-step manual procedures
- Testnet/marketplace dependencies

**Conversion Strategy:**
- **Don't convert** - keep as documentation
- Link from recipes where relevant

**Effort:** N/A

---

## Recommendations by Priority

### Priority 1: Convert EASY Tutorials (1-2 weeks)

**8-10 tutorials** that fit CLI perfectly:
- Build Custom Pallet, Pallet Unit Testing, Add Pallets to Runtime
- XCM Fee Estimation, XCM Transfers
- Pay Fees Different Token
- Fork Live Chains
- Create Smart Contract

**No blockers** - convert immediately

---

### Priority 2: Enhance CLI, Convert MEDIUM Tutorials (1-2 months)

**8-10 tutorials** needing CLI improvements:
- Remark Tutorial (remove browser wallet)
- Fast-Track Governance (convert to PAPI)
- Set Up Template (binary automation)
- Pallet Benchmarking (tool automation)
- Runtime Upgrade (PAPI submission)
- Spawn Basic Chain (Zombienet automation)
- Test & Deploy Hardhat (binary automation)
- Asset Conversion (UI to PAPI)
- Replay & Dry-Run XCMs (compilation automation)

**Blockers:** Dependency management, GUI-to-code conversion

---

### Priority 3: New Pathways, Convert HARD Tutorials (2-3 months)

**6-8 tutorials** needing major work:
- Deploy ERC-20, Deploy NFT (Remix → Hardhat)
- Create dApp Ethers.js, Create dApp Viem (Frontend pathway)
- Deploying Uniswap V2 (multi-binary automation)
- Register Foreign Asset, Register Local Asset (multi-chain + PAPI)

**Blockers:** Missing pathways, complex setups

---

### Don't Convert

**4-5 tutorials** that don't fit:
- Para-to-Para HRMP Channels
- Para-to-System HRMP Channels
- Deploy to Testnet
- Obtain Coretime

**Reason:** Infrastructure guides, not development tutorials

**60-65 guides** that are reference docs - keep as documentation

---

## Key Takeaways

1. **Only 30-37% of tutorials** can convert immediately with existing CLI
2. **37% have GUI dependencies** requiring major refactoring
3. **55% need external services** - recipes can't be fully isolated
4. **80-87% of guides** are reference docs, not executable code
5. **Realistic timeline:** 3-6 months for phased conversion
6. **Realistic goal:** 22-28 working recipes (not all 102 items)

**Bottom line:** Selective conversion with phased approach is the only viable strategy.
