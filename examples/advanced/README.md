# Advanced Examples

This folder contains examples of more complex TypeScript constructs that demonstrate advanced features of the compiler.

## Files

-   `separate_interface_test.ts` - Interface definitions (generate traits)
-   `user_interface.ts` - Exported interface example

## What Works Here

✅ **Interfaces**: Complete interface definitions with methods and properties
✅ **Export statements**: `export interface` declarations
✅ **Type annotations**: Complex type definitions

## Current Limitations

❌ **Import statements**: `import` statements are parsed but not fully processed
❌ **Class inheritance**: `implements` clauses in classes need more work

## Usage

```bash
# Compile interface examples
cargo run -- --input examples/advanced/separate_interface_test.ts --output output.rs
```

## Output

Interfaces are converted to Rust traits with proper method signatures and type mappings.
