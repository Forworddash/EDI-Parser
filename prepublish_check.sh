#!/bin/bash

# EDI Parser - Pre-Publication Checklist
# Run this script before publishing to crates.io

echo "🔍 EDI Parser Pre-Publication Checklist"
echo "======================================"

# Check if all tests pass
echo "🧪 Running tests..."
if ! cargo test --release; then
    echo "❌ Tests failed. Fix them before publishing."
    exit 1
fi
echo "✅ All tests passed"

# Check if documentation builds
echo "📚 Building documentation..."
if ! cargo doc --no-deps; then
    echo "❌ Documentation build failed."
    exit 1
fi
echo "✅ Documentation built successfully"

# Check for warnings
echo "⚠️  Checking for warnings..."
WARNINGS=$(cargo check 2>&1 | grep -c "warning:")
if [ "$WARNINGS" -gt 0 ]; then
    echo "⚠️  Found $WARNINGS warnings. Consider fixing them:"
    cargo check
fi

# Check package metadata
echo "📦 Checking package metadata..."
if ! cargo package --list; then
    echo "❌ Package check failed."
    exit 1
fi

# Check if package can be built from scratch
echo "🔨 Testing clean build..."
if ! cargo clean && cargo build --release; then
    echo "❌ Clean build failed."
    exit 1
fi
echo "✅ Clean build successful"

# Check for required files
echo "📄 Checking required files..."
REQUIRED_FILES=(
    "README.md"
    "LICENSE"
    "Cargo.toml"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file exists"
    else
        echo "❌ $file missing"
    fi
done

echo ""
echo "🎯 Pre-Publication Checklist Complete!"
echo ""
echo "📋 Next Steps:"
echo "1. Update version in Cargo.toml if needed"
echo "2. Commit all changes: git add . && git commit -m 'Prepare for v0.1.0'"
echo "3. Create git tag: git tag v0.1.0"
echo "4. Push to GitHub: git push && git push --tags"
echo "5. Publish to crates.io: cargo publish"
echo ""
echo "📖 After publishing:"
echo "- Check https://crates.io/crates/edi-parser"
echo "- Check https://docs.rs/edi-parser"
echo "- Update README badges if desired"
