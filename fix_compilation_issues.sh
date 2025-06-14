#!/bin/bash

echo "=== Comprehensive Compilation Fix ==="
echo "Date: $(date)"
echo

cd /Users/mariano/Desktop/Code/nano-messenger

echo "1. Fixing Blake2b type annotation issues in encryption.rs..."

# Fix Blake2b usage with proper type annotations
sed -i.backup '
/let mut hasher = Blake2b::new();/c\
        let mut hasher = Blake2b512::new();
/let hash = hasher.finalize();/c\
        let hash = hasher.finalize();
' src/media/security/encryption.rs

echo "2. Fixing HIPAA enum trait implementations..."

# Add Hash, Eq, PartialEq derives to AccessPurpose
sed -i.backup '
/^#\[derive.*\] *$/,/^pub enum AccessPurpose/ {
    /^#\[derive.*\] *$/ s/derive.*$/derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)/
}
' src/media/compliance/hipaa.rs

# Add Hash, Eq, PartialEq derives to UserRole
sed -i.backup '
/^#\[derive.*\] *$/,/^pub enum UserRole/ {
    /^#\[derive.*\] *$/ s/derive.*$/derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)/
}
' src/media/compliance/hipaa.rs

# Add Hash, Eq, PartialEq derives to PHIType  
sed -i.backup '
/^#\[derive.*\] *$/,/^pub enum PHIType/ {
    /^#\[derive.*\] *$/ s/derive.*$/derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)/
}
' src/media/compliance/hipaa.rs

# Add Hash, Eq, PartialEq derives to HIPAAIdentifier
sed -i.backup '
/^#\[derive.*\] *$/,/^pub enum HIPAAIdentifier/ {
    /^#\[derive.*\] *$/ s/derive.*$/derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)/
}
' src/media/compliance/hipaa.rs

echo "3. Fixing GDPR enum trait implementations..."

# Add Hash, Eq, PartialEq derives to ErasureMethod
sed -i.backup '
/^#\[derive.*\] *$/,/^pub enum ErasureMethod/ {
    /^#\[derive.*\] *$/ s/derive.*$/derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)/
}
' src/media/compliance/gdpr.rs

# Add Hash, Eq, PartialEq derives to PersonalDataCategory
sed -i.backup '
/^#\[derive.*\] *$/,/^pub enum PersonalDataCategory/ {
    /^#\[derive.*\] *$/ s/derive.*$/derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)/
}
' src/media/compliance/gdpr.rs

echo "4. Fixing Auditing enum trait implementations..."

# Add Hash, Eq, PartialEq derives to ThreatLevel
sed -i.backup '
/^#\[derive.*\] *$/,/^pub enum ThreatLevel/ {
    /^#\[derive.*\] *$/ s/derive.*$/derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)/
}
' src/media/compliance/auditing.rs

# Add Hash, Eq, PartialEq derives to ComplianceRegulation
sed -i.backup '
/^#\[derive.*\] *$/,/^pub enum ComplianceRegulation/ {
    /^#\[derive.*\] *$/ s/derive.*$/derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)/
}
' src/media/compliance/auditing.rs

echo "5. Fixing Duration::from_minutes issues..."

# Replace Duration::from_minutes with proper Duration::from_secs
sed -i.backup 's/Duration::from_minutes(\([0-9]*\))/Duration::from_secs(\1 * 60)/g' src/media/compliance/auditing.rs

echo "6. Fixing mutable reference issues..."

# Fix mutable reference issues in mod.rs
sed -i.backup 's/fn assess_media_compliance(/fn assess_media_compliance(\&mut self,/g' src/media/compliance/mod.rs
sed -i.backup 's/fn establish_media_session(/fn establish_media_session(\&mut self,/g' src/media/security/mod.rs
sed -i.backup 's/fn check_media_access(/fn check_media_access(\&mut self,/g' src/media/security/access_control.rs
sed -i.backup 's/fn distribute_quantum_keys(/fn distribute_quantum_keys(\&mut self,/g' src/media/security/encryption.rs
sed -i.backup 's/fn add_phi_event(/fn add_phi_event(\&mut self,/g' src/media/compliance/hipaa.rs

echo "7. Fixing type mismatches..."

# Fix array to vector conversion issues
sed -i.backup 's/current_hash: \[0u8; 64\]/current_hash: vec![0u8; 64]/g' src/media/security/forensics.rs
sed -i.backup 's/chain_hash: \[0u8; 64\]/chain_hash: vec![0u8; 64]/g' src/media/security/forensics.rs

echo "8. Fixing float type annotations..."

# Fix ambiguous float type
sed -i.backup 's/let mut max_confidence = 0.0;/let mut max_confidence: f64 = 0.0;/g' src/media/compliance/gdpr.rs

echo "9. Adding missing Serialize derive for ActionPriority..."

# Add Serialize derive to ActionPriority if it exists
if grep -q "pub enum ActionPriority" src/media/compliance/mod.rs; then
    sed -i.backup '
    /^#\[derive.*\] *$/,/^pub enum ActionPriority/ {
        /^#\[derive.*\] *$/ s/derive.*$/derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)/
    }
    ' src/media/compliance/mod.rs
fi

echo "10. Fixing Blake2b hasher issues in auditing.rs..."

# Fix Blake2b usage in auditing.rs
sed -i.backup '
/let mut hasher = Blake2b::new();/c\
        let mut hasher = Blake2b512::new();
' src/media/compliance/auditing.rs

echo "11. Fixing Regulation enum Hash derive..."

# Add Hash derive to Regulation enum
sed -i.backup '
/^#\[derive.*\] *$/,/^pub enum Regulation/ {
    /^#\[derive.*\] *$/ s/derive.*$/derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)/
}
' src/media/compliance/mod.rs

echo "12. Running a compilation test..."
cargo check --lib 2>&1 | head -30

echo
echo "=== Fix Applied ==="
echo "Summary of changes made:"
echo "- Fixed Blake2b type annotations"
echo "- Added Hash, Eq, PartialEq, Ord derives to enums"
echo "- Fixed Duration::from_minutes to Duration::from_secs"
echo "- Fixed mutable reference issues"
echo "- Fixed type mismatches"
echo "- Fixed float type annotation"
echo
echo "Please review the compilation output above."
