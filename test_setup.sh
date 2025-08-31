#!/bin/bash

# EDI Parser Test Script
# This script helps verify that your EDI parser setup is working correctly

echo "🧪 EDI Parser Test Suite"
echo "========================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the EDI-Parser directory"
    exit 1
fi

echo "📁 Current directory: $(pwd)"
echo ""

# Check Rust installation
echo "🔧 Checking Rust installation..."
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust version: $(rustc --version)"
echo "✅ Cargo version: $(cargo --version)"
echo ""

# Build the project
echo "🏗️  Building project..."
if ! cargo build; then
    echo "❌ Build failed"
    exit 1
fi
echo "✅ Build successful"
echo ""

# Run tests
echo "🧪 Running tests..."
if ! cargo test; then
    echo "❌ Some tests failed"
    exit 1
fi
echo "✅ All tests passed"
echo ""

# Test examples
echo "📝 Testing examples..."

echo "  - Basic parser example..."
if cargo run --example basic_parser > /dev/null 2>&1; then
    echo "  ✅ Basic parser example works"
else
    echo "  ❌ Basic parser example failed"
fi

echo "  - Extended 850 parser example..."
if cargo run --example extended_850_parser > /dev/null 2>&1; then
    echo "  ✅ Extended 850 parser example works"
else
    echo "  ❌ Extended 850 parser example failed"
fi

echo "  - Loop parsing demo..."
if cargo run --example loop_parsing_demo > /dev/null 2>&1; then
    echo "  ✅ Loop parsing demo works"
else
    echo "  ❌ Loop parsing demo failed"
fi

echo ""

# Check test files
echo "📄 Checking test files..."
test_files=(
    "tests/test_files/sample_810.edi"
    "tests/test_files/sample_850.edi"
    "tests/test_files/sample_850_extended.edi"
    "tests/test_files/invalid_sample.edi"
)

for file in "${test_files[@]}"; do
    if [ -f "$file" ]; then
        echo "  ✅ $file exists"
    else
        echo "  ❌ $file missing"
    fi
done

echo ""

# Show usage examples
echo "📚 Usage Examples:"
echo "=================="
echo ""
echo "1. Parse an EDI file:"
echo "   cargo run --example basic_parser"
echo ""
echo "2. Run all tests:"
echo "   cargo test"
echo ""
echo "3. Run specific test:"
echo "   cargo test test_850_extended_parsing"
echo ""
echo "4. Add your own test file:"
echo "   - Place EDI file in tests/test_files/"
echo "   - Add test function in tests/integration_tests.rs"
echo "   - Run: cargo test your_test_name"
echo ""
echo "5. Create a new example:"
echo "   - Add .rs file in examples/"
echo "   - Run: cargo run --example your_example"
echo ""

echo "🎉 Setup verification complete!"
echo "Your EDI parser is ready to use."
