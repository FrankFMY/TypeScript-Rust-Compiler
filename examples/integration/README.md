# Integration Examples

This folder contains comprehensive integration tests that demonstrate the limits of the current compiler implementation.

## Files

-   `comprehensive_test.ts` - Large test file with many TypeScript features (513+ lines)
-   `import_interface_test.ts` - Test of import/export functionality

## What This Tests

This folder contains tests that push the boundaries of what the compiler can currently handle:

🔍 **Comprehensive TypeScript Features**:

-   All basic types and constructs
-   Complex object structures
-   Advanced type system features
-   Module system (import/export)

## Current Status

⚠️ **Mixed Results**: Some features work, others are still being developed

### ✅ Working

-   Basic variable declarations
-   Simple function definitions
-   Basic class structures
-   Interface definitions
-   Enum declarations

### ❌ Not Yet Working

-   Import/export processing
-   Complex inheritance (`implements`)
-   Generic type parameters
-   Advanced type system features
-   Module resolution

## Usage

```bash
# Test comprehensive features (will show what's working vs not working)
cargo run -- --input examples/integration/comprehensive_test.ts --output output.rs

# Test import functionality
cargo run -- --input examples/integration/import_interface_test.ts --output output.rs
```

## Analysis

These tests help identify:

1. What features are fully implemented
2. What features need more work
3. What features are completely missing
4. Performance characteristics with larger files
