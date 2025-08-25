# EDI Parser Architecture Guide

## 🏗️ **Recommended Structure for Comprehensive EDI Parsing**

### **1. Core Architecture**

```
src/
├── error.rs                    # Centralized error handling
├── lib.rs                      # Main library exports
├── models/                     # Basic EDI data structures
│   ├── mod.rs
│   ├── segment.rs             # Raw segment representation
│   ├── transaction.rs         # Raw transaction container
│   └── interchange.rs         # EDI interchange structure
├── parsers/                   # EDI parsing engines
│   ├── mod.rs
│   ├── common.rs              # Parser trait
│   ├── x12.rs                 # X12 parser implementation
│   └── builder.rs             # Configuration builder
├── segments/                  # Structured segment types
│   ├── mod.rs
│   ├── control.rs             # ISA, GS, ST, SE segments
│   ├── common.rs              # N1, REF, DTM segments
│   └── purchase_order.rs      # BEG, PO1, CTT segments
├── transactions/              # Business document structures
│   ├── mod.rs
│   ├── purchase_order_850.rs  # Complete 850 PO structure
│   └── invoice_810.rs         # 810 Invoice (placeholder)
└── utils/                     # Helper utilities
```

### **2. Segment Organization Strategy**

#### **By Functionality:**
- **`control.rs`** - Envelope segments (ISA, IEA, GS, GE, ST, SE)
- **`common.rs`** - Reusable business segments (N1, REF, DTM, etc.)
- **`purchase_order.rs`** - 850-specific segments (BEG, PO1, CTT)
- **`invoice.rs`** - 810-specific segments (BIG, IT1, TDS)
- **`acknowledgment.rs`** - 855-specific segments

#### **Key Segments for 850 Purchase Orders:**
```rust
// Required segments
ST   - Transaction Set Header
BEG  - Beginning Segment for Purchase Order  
SE   - Transaction Set Trailer

// Common optional segments
N1   - Name/Address
REF  - Reference Identification
DTM  - Date/Time Reference
PO1  - Baseline Item Data
CTT  - Transaction Totals
```

### **3. Adding New Segment Types**

To add support for any segment, follow this pattern:

```rust
// In appropriate segments/*.rs file
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct YourSegment {
    pub field1: String,
    pub field2: Option<String>,
    // ... map each element position to a meaningful field
}

impl SegmentParser for YourSegment {
    type Output = Self;
    
    fn parse_segment(segment: &Segment) -> Result<Self::Output, EdiError> {
        // Validate segment ID and element count
        // Map elements to struct fields
    }
    
    fn segment_id() -> &'static str { "XXX" }
    
    fn validate(&self) -> Result<(), EdiError> {
        // Business rule validation
    }
}
```

### **4. Transaction Document Structure**

For complete business documents (like 850 PO):

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct PurchaseOrder850 {
    pub header: StSegment,           // ST segment
    pub beginning_segment: BegSegment, // BEG segment
    pub parties: Vec<N1Segment>,     // All N1 segments
    pub line_items: Vec<Po1Segment>, // All PO1 segments
    pub totals: Option<CttSegment>,  // CTT if present
    pub trailer: Option<SeSegment>,  // SE segment
}
```

### **5. Complete 850 Segment Coverage**

Here are the key segments you should implement for full 850 support:

#### **Essential:**
- ✅ ST (Transaction Set Header) - **Done**
- ✅ BEG (Beginning Segment) - **Done**
- ✅ PO1 (Baseline Item Data) - **Done**
- ✅ CTT (Transaction Totals) - **Done**
- ✅ SE (Transaction Set Trailer) - **Done**

#### **Common:**
- ✅ N1 (Name) - **Done**
- ✅ REF (Reference) - **Done**
- ✅ DTM (Date/Time) - **Done**

#### **Additional Segments to Add:**
```rust
// Address details
N2   - Additional Name Information
N3   - Address Information  
N4   - Geographic Location

// Terms and conditions
TD5  - Carrier Details
ITD  - Terms of Sale/Deferred Terms
SAC  - Service, Promotion, Allowance

// Item details
PID  - Product/Item Description
MEA  - Measurements
QTY  - Quantity
```

### **6. Usage Pattern**

```rust
// 1. Parse EDI interchange
let interchange = parser.parse(edi_data)?;

// 2. Extract transactions
for transaction in &interchange.functional_groups[0].transactions {
    if transaction.transaction_set_id == "850" {
        // 3. Parse into structured format
        let po = PurchaseOrder850::parse_transaction(transaction)?;
        
        // 4. Access business data
        println!("PO#: {}", po.purchase_order_number());
        println!("Total: ${:.2}", po.total_value().unwrap_or(0.0));
        
        // 5. Validate business rules
        po.validate()?;
    }
}
```

### **7. Benefits of This Architecture**

✅ **Type Safety** - Each segment has strongly typed fields  
✅ **Validation** - Business rule validation at segment and document level  
✅ **Extensibility** - Easy to add new segment types and transaction types  
✅ **Maintainability** - Clear separation of concerns  
✅ **Performance** - Parse only what you need  
✅ **Documentation** - Self-documenting through type names  

### **8. Next Steps**

1. **Add remaining 850 segments** (N2, N3, N4, TD5, etc.)
2. **Implement 810 Invoice parsing** following the same pattern
3. **Add validation rules** for business logic
4. **Consider performance optimizations** (zero-copy parsing)
5. **Add error recovery** for malformed segments

This architecture gives you a solid foundation to parse any EDI document type with full type safety and validation!
