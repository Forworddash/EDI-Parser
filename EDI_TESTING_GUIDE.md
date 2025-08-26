# 🧪 Testing Random EDI Files

## Quick Start

You now have two powerful tools to test any EDI file:

### 1. File-Based Tester
Test any EDI file by providing the file path:

```bash
cargo run --example edi_file_tester <path_to_edi_file>
```

**Examples:**
```bash
# Test a sample file
cargo run --example edi_file_tester tests/test_files/sample_850.edi

# Test your own file
cargo run --example edi_file_tester "C:\path\to\your\edi\file.edi"
cargo run --example edi_file_tester "C:\path\to\your\edi\file.x12"
```

### 2. Interactive Tester
Paste EDI content directly and get immediate analysis:

```bash
cargo run --example edi_interactive_tester
```

Then paste your EDI content and press Enter twice when done.

## What the Tester Shows You

### ✅ Basic Parsing Test
- **Structure Validation**: Checks ISA/GS/ST/SE/GE/IEA envelope structure
- **Sender/Receiver Info**: Extracts trading partner IDs
- **Transaction Count**: Shows number of functional groups and transactions
- **Performance**: Displays parsing time

### 🔬 Schema Validation Test  
- **Compliance Check**: Validates against X12 schema rules
- **Element Validation**: Checks element requirements and data types
- **Error Reporting**: Shows validation errors and warnings
- **Transaction Analysis**: Identifies transaction types and segment counts

### 🔍 Element ID Tracking
- **Element Mapping**: Shows "CUR01 = Element 98" style mappings
- **Business Data**: Extracts key business information:
  - Currency codes (CUR segments)
  - Purchase order numbers (BEG segments)
  - Reference numbers (REF segments)
  - Line item details (PO1 segments)
  - Name/address data (N1 segments)

### 📋 Multi-Version Compatibility
- **Version Testing**: Tests against X12 versions 004010, 005010, 008010
- **Compatibility Report**: Shows which versions work with your file

## Example Output

```
🧪 EDI File Tester - Test Any EDI File
======================================

📁 Testing file: tests/test_files/sample_850.edi
📊 File size: 326 bytes

🔍 Test 1: Basic X12 Parsing
============================
✅ Basic parsing successful!
   📊 Sender ID: BUYERID
   📊 Receiver ID: SELLERID
   📊 Total Transactions: 1

🔍 Test 3: Element ID Tracking
==============================
✅ Element tracking analysis:
   📄 Purchase Order: BEG01='00' (Element 353), BEG03='PO-001' (Element 324)
   🏢 Name: N101='ST' (Element 98), N102='Supplier Inc' (Element 93)
   📦 Item: PO101='1', PO102='100' (Element 330), PO104='10.50' (Element 212)
```

## Common File Types Supported

- `.edi` - Standard EDI files
- `.x12` - X12 transaction files  
- `.txt` - Text files with EDI content
- Any text file containing EDI data

## Troubleshooting

### ❌ Parsing Failed
- **Check segment terminators**: Should end with `~`
- **Verify element separators**: Should use `*`
- **Ensure proper structure**: Must have ISA/GS/ST/SE/GE/IEA envelope

### ⚠️ Validation Warnings
- **Unknown segments**: Segments not defined in schema
- **Missing elements**: Required elements not present
- **Invalid codes**: Element values not in valid code list

### 🔍 Element Not Found
- **Segment missing**: Business segment (CUR, REF, BEG, etc.) not in file
- **Insufficient elements**: Segment has fewer elements than expected
- **Version mismatch**: Element definitions vary by X12 version

## Tips for Testing

1. **Start with file tester** for comprehensive analysis
2. **Use interactive tester** for quick validation of small EDI snippets
3. **Check all four test sections** for complete analysis
4. **Pay attention to validation results** for compliance issues
5. **Try different X12 versions** if one doesn't work

## Getting More Details

For detailed element definitions and schema information, see:
- `ELEMENT_ID_SYSTEM.md` - Element ID cross-reference
- `SCHEMA_EXTENSION_GUIDE.md` - Schema and validation details
- `README.md` - Complete parser documentation

Your EDI parser now provides comprehensive testing for any EDI file! 🎉
