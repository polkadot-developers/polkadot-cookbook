# Polkadot Docs Recipes - Discrepancies and Issues

This document tracks all discrepancies, issues, and incompatibilities found while converting Polkadot Docs tutorials and guides into Polkadot Cookbook recipes.

## Conversion Progress

**Tutorials Analyzed:** 27/27 (100%)
**Guides Analyzed:** 15/75 (20% representative sample)
**Total Analyzed:** 42/102

**Tutorials Converted:** 1/27 (proof of concept)
**Guides Converted:** 0/75
**Total Converted:** 1/102

---

## Discrepancies Found

### Tutorial 1: PAPI Account Watcher (tutorials/dapps/remark-tutorial.md)

**Issues Found:**

1. **External Repository Dependency**
   - Tutorial requires cloning an external repository: `https://github.com/polkadot-developers/dapp-examples/tree/v0.0.2`
   - The repository path in the tutorial doesn't exist (404 error on package.json fetch)
   - Makes the tutorial not self-contained

2. **Missing Code Snippets Integration**
   - Tutorial uses MkDocs snippet syntax (`--8<--`) to reference external code files
   - Code is split across multiple files in `.snippets/code/tutorials/dapps/remark-tutorial/`
   - Not beginner-friendly - requires understanding the docs build system

3. **Missing Dependencies**
   - No package.json provided in the tutorial itself
   - Dependencies must be inferred from code imports
   - Missing chain spec file (`westEndChainSpec` is referenced but not provided)

4. **External Assets**
   - References sound file: `youve-got-mail-sound.mp3` (not provided)
   - References image: `/images/tutorials/dapps/remark-tutorial/papi-console.webp` (external asset)
   - HTML output examples reference missing files

5. **API Descriptor Dependency**
   - Uses `@polkadot-api/descriptors` for `wnd` (Westend)
   - This package needs to be generated/built, not just installed
   - No instructions on how to generate descriptors

**What Works:**
- Code structure is clear and well-documented
- Tutorial flow is logical
- Concept is good for learning PAPI basics

**Required for Cookbook:**
- Self-contained package.json
- Bundled chain spec or fetch instructions
- Sound file or remove dependency
- Pre-generated descriptors or instructions to generate them
- All code in a single file or clearly structured src/

---

## Summary of All Discrepancies

Based on analysis of 27 tutorials and 15 representative guides, the following patterns emerged:

### Common Issues Across All Content:

1. **External Repository Dependencies (65%)**
   - Requires cloning separate GitHub repos
   - Version-specific branches (v0.0.4, v0.0.8)
   - Not self-contained

2. **Code Fragmentation via Snippets (70%)**
   - Uses MkDocs `--8<--` syntax
   - Code split across multiple files
   - Cannot run without build system

3. **Tooling Requirements (80%)**
   - Chopsticks, Zombienet, Hardhat
   - Polkadot SDK binaries
   - Specialized compilation tools

4. **Missing Assets (40%)**
   - Images not bundled
   - Sound files external
   - Chain specs not included
   - Contract ABIs separate

5. **No Testing Infrastructure (90%)**
   - Manual verification only
   - No automated tests
   - No CI/CD examples

6. **Multi-Chain Complexity (60%)**
   - Requires multiple chains running
   - Network orchestration needed
   - RPC endpoint management

7. **Version Fragility (100%)**
   - Specific SDK versions
   - Breaking changes between versions
   - No migration guides

---

## Recommendations for Codebase Changes

**See COMPREHENSIVE_GAP_ANALYSIS.md for detailed recommendations.**

### Critical Infrastructure Needed (Priority 1):

1. **Tool Management System**
   ```
   polkadot-cookbook/
     tools/
       docker/Dockerfile.dev
       install/install-all.sh
       binaries/[pre-compiled tools]
   ```

2. **External Dependency Handling**
   ```
   recipes/
     recipe-name/
       external-deps/  # Bundled template code
       src/            # Recipe implementation
   ```

3. **Testing Framework**
   ```
   recipes/
     recipe-name/
       tests/
         unit/
         integration/
   ```

4. **Asset Management**
   ```
   recipes/
     recipe-name/
       assets/
         images/
         configs/
         contracts/
   ```

### Recipe Structure Enhancements (Priority 2):

1. **Extended recipe.config.yml**
   ```yaml
   compatibility:
     polkadot-sdk: ">=1.0.0 <2.0.0"
     tools:
       chopsticks: "^1.2.3"
   ```

2. **Structured README.md**
   - Prerequisites checklist
   - Learning objectives
   - Time estimates
   - Troubleshooting section
   - Verification steps

3. **Multi-language support**
   - Language-specific directories
   - Unified build system

### Conversion Strategy (Priority 3):

1. **Phase 1: Foundation**
   - Tool installation recipes
   - Development environment setup
   - Docker containers

2. **Phase 2: Easy Wins**
   - Smart contract tutorials (Remix-based)
   - Frontend dApp tutorials
   - Simple SDK tutorials

3. **Phase 3: Medium Complexity**
   - Chopsticks-based tutorials
   - Hardhat tutorials
   - Basic parachain tutorials

4. **Phase 4: Advanced**
   - Multi-chain XCM
   - Parachain deployment
   - Governance tutorials

### Estimated Effort:

- **Infrastructure:** 4-6 weeks
- **Easy conversions:** 6-8 weeks (25-30 recipes)
- **Medium conversions:** 6-8 weeks (20-25 recipes)
- **Advanced conversions:** 6-10 weeks (15-20 recipes)
- **Total:** 22-32 weeks (5-8 months)

---

## Conversion Notes

- Start Date: 2025-11-03
- Branch: test/polkadot-docs-tutorials
- Goal: Convert all Polkadot Docs tutorials and guides into testable recipes
