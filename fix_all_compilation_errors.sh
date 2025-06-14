#!/bin/bash

# 🔧 COMPREHENSIVE COMPILATION FIX
# Fixes the remaining compilation errors before Session 19

set -euo pipefail

PROJECT_ROOT="/Users/mariano/Desktop/Code/nano-messenger"
BACKUP_DIR="${PROJECT_ROOT}/compilation_fix_backup_$(date +%Y%m%d_%H%M%S)"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🔧 COMPREHENSIVE COMPILATION FIX${NC}"
echo -e "=================================="

cd "$PROJECT_ROOT" || exit 1

# Create backup
echo -e "${YELLOW}📦 Creating backup...${NC}"
mkdir -p "$BACKUP_DIR"
cp -r src "$BACKUP_DIR/"
echo -e "${GREEN}✅ Backup created at: $BACKUP_DIR${NC}"

# Fix 1: CryptoMode::QuantumSafe pattern matching
echo -e "${YELLOW}🔧 Fixing CryptoMode::QuantumSafe pattern matching...${NC}"
sed -i '' 's/CryptoMode::Quantum => "ML-KEM-768+ML-DSA-65".to_string(),/CryptoMode::Quantum | CryptoMode::QuantumSafe => "ML-KEM-768+ML-DSA-65".to_string(),/' src/media/encryption.rs

# Fix 2: Add missing QuantumSafe pattern in encrypt_file_key method
sed -i '' '/CryptoMode::Quantum, UnifiedPublicKeys::PostQuantum/i\
            (CryptoMode::QuantumSafe, UnifiedPublicKeys::PostQuantum(keys)) => {\
                PostQuantumAsymmetricEncryption::encrypt(&keys.public_key, file_key)\
            }\
            (CryptoMode::QuantumSafe, UnifiedPublicKeys::Classical(keys)) => {\
                ClassicalAsymmetricEncryption::encrypt(&keys.x25519_key, file_key)\
            }\
            (CryptoMode::QuantumSafe, UnifiedPublicKeys::Hybrid(keys)) => {\
                let hybrid_public_key = crate::crypto::hybrid::HybridPublicKey {\
                    classical: keys.classical.x25519_key,\
                    post_quantum: keys.post_quantum.public_key.clone(),\
                };\
                HybridAsymmetricEncryption::encrypt(&hybrid_public_key, file_key)\
            }\
' src/media/encryption.rs

# Fix 3: Fix unused variable warnings by adding underscore prefix
echo -e "${YELLOW}🔧 Fixing unused variable warnings...${NC}"
sed -i '' 's/user_id: &UserId/_user_id: \&UserId/g' src/media/security/scanning.rs

# Fix 4: Clean up any remaining sed command issues in other files
echo -e "${YELLOW}🔧 Checking for other pattern matching issues...${NC}"

# Fix any other CryptoMode pattern matches that might be missing QuantumSafe
find src -name "*.rs" -exec grep -l "match.*crypto_mode\|match.*CryptoMode" {} \; | while read -r file; do
    if grep -q "CryptoMode::Quantum =>" "$file" && ! grep -q "CryptoMode::QuantumSafe" "$file"; then
        echo "  Fixing pattern match in: $file"
        sed -i '' 's/CryptoMode::Quantum =>/CryptoMode::Quantum | CryptoMode::QuantumSafe =>/g' "$file"
    fi
done

# Fix 5: Temporarily disable ffmpeg-next if still causing issues
if cargo check --lib 2>&1 | grep -q "ffmpeg-next\|AVPacketSideDataType"; then
    echo -e "${YELLOW}🔧 Disabling ffmpeg-next dependency...${NC}"
    sed -i '' 's/^ffmpeg-next = .*/# ffmpeg-next = { version = "6.0", optional = true }  # TEMPORARILY DISABLED/' Cargo.toml
    sed -i '' 's/^video-processing = .*/# video-processing = ["ffmpeg-next"]   # TEMPORARILY DISABLED/' Cargo.toml
    sed -i '' 's/media-full = \[.*\]/media-full = ["image-processing", "exif-processing"]/' Cargo.toml
fi

# Clean previous build
echo -e "${YELLOW}🧹 Cleaning previous build...${NC}"
cargo clean >/dev/null 2>&1

# Test compilation
echo -e "${YELLOW}🧪 Testing compilation...${NC}"
if cargo check --lib; then
    echo -e "${GREEN}✅ Library compilation successful!${NC}"
else
    echo -e "${RED}❌ Still having compilation issues${NC}"
    exit 1
fi

# Test with features
echo -e "${YELLOW}🧪 Testing with core features...${NC}"
if cargo check --features="local-storage,image-processing,session11-basic,compliance-basic"; then
    echo -e "${GREEN}✅ Core features compilation successful!${NC}"
else
    echo -e "${RED}❌ Core features compilation failed${NC}"
    exit 1
fi

echo -e "\n${GREEN}🎉 ALL COMPILATION ERRORS FIXED!${NC}"
echo -e "${BLUE}📋 Changes made:${NC}"
echo -e "   • Fixed CryptoMode::QuantumSafe pattern matching"
echo -e "   • Added missing QuantumSafe cases in encryption methods"
echo -e "   • Fixed unused variable warnings"
echo -e "   • Disabled ffmpeg-next if needed"
echo -e "   • Cleaned build artifacts"

echo -e "\n${GREEN}✅ Ready for Session 19 cleanup!${NC}"
echo -e "Run: ${BLUE}chmod +x session19_final_cleanup.sh && ./session19_final_cleanup.sh${NC}"
