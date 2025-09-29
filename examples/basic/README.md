# Basic Examples

This folder contains examples of basic TypeScript constructs that the compiler can successfully handle.

## Files

-   `simple_test.ts` - Basic variables, arrays, and objects
-   `working_features_test.ts` - Comprehensive test of all currently working features
-   `function_test.ts` - Function declarations and calls
-   `enum_test.ts` - Enum declarations with different value types
-   `simple_class_test.ts` - Simple class without inheritance

## What Works Here

✅ **Variables**: `let`, `const`, `var` with type annotations
✅ **Arrays**: `number[]`, `Array<string>`, etc.
✅ **Objects**: Object literals with type annotations
✅ **Functions**: Function declarations with parameters and return types
✅ **Enums**: Basic enums and string enums
✅ **Classes**: Simple classes with properties and methods
✅ **Interfaces**: Interface declarations (generate traits)

## Usage

```bash
# Compile any of these files
cargo run -- --input examples/basic/simple_test.ts --output output.rs
```

## Output

Each file generates corresponding Rust code with proper type mappings and struct/trait generation.
