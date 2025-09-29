# TypeScript-Rust-Compiler Examples

This folder contains examples and test files for the TypeScript-Rust-Compiler, organized by complexity and feature coverage.

## Structure

```
examples/
├── basic/          # Basic TypeScript constructs that work
├── advanced/       # More complex features (partial support)
├── integration/    # Comprehensive tests (mixed results)
└── README.md       # This file
```

## Categories

### 🟢 Basic Examples (`basic/`)

**Status**: ✅ **Fully Working**

Examples of TypeScript constructs that the compiler handles completely:

-   Variables with type annotations
-   Arrays and objects
-   Functions with parameters and return types
-   Simple classes
-   Basic enums
-   Interface definitions

### 🟡 Advanced Examples (`advanced/`)

**Status**: ⚠️ **Partially Working**

More complex TypeScript features with some limitations:

-   Interface definitions (work well)
-   Export statements
-   Complex type annotations

### 🔴 Integration Examples (`integration/`)

**Status**: ⚠️ **Mixed Results**

Comprehensive tests that push the boundaries:

-   Large files with many features
-   Import/export statements
-   Complex inheritance
-   Advanced type system features

## Quick Start

```bash
# Test basic functionality
cargo run -- --input examples/basic/simple_test.ts --output output.rs

# Test interface generation
cargo run -- --input examples/advanced/separate_interface_test.ts --output output.rs

# Test comprehensive features
cargo run -- --input examples/integration/comprehensive_test.ts --output output.rs
```

## Understanding the Output

Each example generates Rust code that demonstrates:

1. **Type Mapping**: How TypeScript types become Rust types
2. **Struct Generation**: Classes become structs with impl blocks
3. **Trait Generation**: Interfaces become traits
4. **Enum Generation**: TypeScript enums become Rust enums
5. **Function Translation**: TypeScript functions become Rust functions

## Current Limitations

The compiler is still under development. Some TypeScript features are not yet fully supported:

-   Complex inheritance (`implements` clauses)
-   Generic type parameters
-   Import/export resolution
-   Advanced type system features
-   Module system

## Contributing

When adding new examples:

1. Place basic working examples in `basic/`
2. Place partially working examples in `advanced/`
3. Place comprehensive tests in `integration/`
4. Update the relevant README.md
5. Test your examples work as expected

## See Also

-   [Main README](../README.md) - Project overview
-   [Integration Tests](../tests/) - Unit and integration tests
