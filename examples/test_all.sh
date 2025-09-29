#!/bin/bash

# Script to test all examples in the examples directory

echo "🧪 Testing TypeScript-Rust-Compiler Examples"
echo "============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test basic examples
echo -e "\n${GREEN}Testing Basic Examples...${NC}"
for file in examples/basic/*.ts; do
    if [ -f "$file" ]; then
        filename=$(basename "$file" .ts)
        echo -e "\n  Testing: ${YELLOW}$filename.ts${NC}"

        # Create output filename
        output="examples_outputs/${filename}_output.rs"

        # Compile
        if cargo run -- --input "$file" --output "$output" > /dev/null 2>&1; then
            echo -e "    ${GREEN}✅ Success${NC}"
            echo "    Output: $output"
        else
            echo -e "    ${RED}❌ Failed${NC}"
        fi
    fi
done

# Test advanced examples
echo -e "\n${YELLOW}Testing Advanced Examples...${NC}"
for file in examples/advanced/*.ts; do
    if [ -f "$file" ]; then
        filename=$(basename "$file" .ts)
        echo -e "\n  Testing: ${YELLOW}$filename.ts${NC}"

        output="examples_outputs/${filename}_output.rs"

        if cargo run -- --input "$file" --output "$output" > /dev/null 2>&1; then
            echo -e "    ${GREEN}✅ Success${NC}"
            echo "    Output: $output"
        else
            echo -e "    ${RED}❌ Failed${NC}"
        fi
    fi
done

# Test integration examples
echo -e "\n${RED}Testing Integration Examples...${NC}"
for file in examples/integration/*.ts; do
    if [ -f "$file" ]; then
        filename=$(basename "$file" .ts)
        echo -e "\n  Testing: ${YELLOW}$filename.ts${NC}"

        output="examples_outputs/${filename}_output.rs"

        if cargo run -- --input "$file" --output "$output" > /dev/null 2>&1; then
            echo -e "    ${GREEN}✅ Success${NC}"
            echo "    Output: $output"
        else
            echo -e "    ${RED}❌ Failed${NC}"
        fi
    fi
done

echo -e "\n${GREEN}Testing complete!${NC}"
echo "Check the examples_outputs/ directory for generated Rust files."
