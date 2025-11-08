---
title: Basic Pallet
description: Replace with a short description.
---

# Basic Pallet

> Replace this with a one-sentence description of what this recipe teaches.

## Overview

<!-- Replace with 2-3 paragraphs explaining:
- What problem this recipe solves
- Who should use this recipe
- What they will learn
-->

## Prerequisites

- Rust 1.81.0 or later
- Basic understanding of FRAME pallets
- Familiarity with Substrate development

## What You'll Learn

<!-- List 3-5 key concepts or skills this recipe teaches -->

- How to create a custom FRAME pallet
- How to implement storage items
- How to emit events
- How to write tests with a mock runtime

## Implementation

### Pallet Structure

<!-- Explain the structure of the pallet -->

The pallet is located in `pallets/template/` and includes:

- **Storage**: Define your storage items
- **Events**: Emit events when state changes occur
- **Errors**: Handle error cases gracefully
- **Dispatchable functions**: Implement callable extrinsics

### Key Code

<!-- Highlight and explain the most important parts of the code -->

```rust
// Example: Explain important code snippets here
#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn store_something(origin: OriginFor<T>, value: u32) -> DispatchResult {
        let who = ensure_signed(origin)?;
        <Something<T>>::put(value);
        Self::deposit_event(Event::SomethingStored { value, who });
        Ok(())
    }
}
```

## Testing

The recipe includes comprehensive tests in the `tests/` directory:

### Running Tests

```bash
# Run all tests
just test

# Run tests with output
just test-verbose

# Run specific test
cargo test it_works_for_default_value
```

### Test Coverage

The tests demonstrate:

- Basic storage operations
- Event emission
- Error handling
- Permission checks (root vs signed origin)

## Building

```bash
# Check code
just check

# Build the pallet
just build

# Run all checks (format, clippy, tests)
just ci
```

## Integration

To use this pallet in your runtime:

1. Add to your runtime's `Cargo.toml`:
```toml
pallet-template = { path = "path/to/this/pallet", default-features = false }
```

2. Include in your runtime's `construct_runtime!` macro:
```rust
construct_runtime!(
    pub enum Runtime {
        // ... other pallets
        TemplateModule: pallet_template,
    }
);
```

3. Implement the pallet's `Config` trait:
```rust
impl pallet_template::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}
```

## Next Steps

<!-- Suggest what readers should explore next -->

- Extend the pallet with additional functionality
- Add benchmarking for weight calculations
- Integrate with other pallets
- Deploy to a local testnet

## Resources

- [FRAME Documentation](https://docs.substrate.io/reference/frame-pallets/)
- [Pallet Development Guide](https://docs.substrate.io/build/custom-pallets/)
- [Substrate Tutorials](https://docs.substrate.io/tutorials/)

## License

MIT OR Apache-2.0
