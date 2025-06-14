#!/bin/bash

# 🧪 Quick validation script to check current project status
# Run this before Session 19 cleanup to see current state

PROJECT_ROOT="/Users/mariano/Desktop/Code/nano-messenger"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🔍 NANO MESSENGER - PRE-SESSION 19 STATUS CHECK${NC}"
echo -e "=================================================="

cd "$PROJECT_ROOT" || exit 1

echo -e "\n${YELLOW}📊 Current Project Status:${NC}"

# Check basic compilation
echo -e "\n${BLUE}1. Basic Compilation Check:${NC}"
if cargo check --lib >/dev/null 2>&1; then
    echo -e "   ✅ Library compilation: ${GREEN}SUCCESS${NC}"
else
    echo -e "   ❌ Library compilation: ${RED}FAILED${NC}"
fi

# Check all features
echo -e "\n${BLUE}2. All Features Check:${NC}"
if cargo check --all-features >/dev/null 2>&1; then
    echo -e "   ✅ All features: ${GREEN}SUCCESS${NC}"
else
    echo -e "   ⚠️  All features: ${YELLOW}WARNINGS PRESENT${NC}"
fi

# Count warnings
echo -e "\n${BLUE}3. Warning Analysis:${NC}"
warning_count=$(cargo check --all-features 2>&1 | grep -c "warning:" || echo "0")
echo -e "   📝 Current warnings: ${YELLOW}$warning_count${NC}"

# Check file count
echo -e "\n${BLUE}4. Source File Count:${NC}"
rs_files=$(find src -name "*.rs" | wc -l | tr -d ' ')
echo -e "   📄 Rust files: ${GREEN}$rs_files${NC}"

# Check features
echo -e "\n${BLUE}5. Feature Testing:${NC}"
features=("session11-basic" "image-processing" "compliance-basic" "local-storage")
for feature in "${features[@]}"; do
    if cargo check --features="$feature" >/dev/null 2>&1; then
        echo -e "   ✅ $feature: ${GREEN}OK${NC}"
    else
        echo -e "   ⚠️  $feature: ${YELLOW}ISSUES${NC}"
    fi
done

# Project structure
echo -e "\n${BLUE}6. Project Structure:${NC}"
key_dirs=("src/crypto" "src/media" "src/production" "src/config")
for dir in "${key_dirs[@]}"; do
    if [[ -d "$dir" ]]; then
        echo -e "   ✅ $dir: ${GREEN}EXISTS${NC}"
    else
        echo -e "   ❌ $dir: ${RED}MISSING${NC}"
    fi
done

# Session tracking
echo -e "\n${BLUE}7. Session Progress:${NC}"
completed_sessions=$(ls SESSION*_COMPLETED.md 2>/dev/null | wc -l | tr -d ' ')
echo -e "   📈 Completed sessions: ${GREEN}$completed_sessions/19${NC}"

echo -e "\n${YELLOW}🚀 Ready for Session 19 cleanup!${NC}"
echo -e "Run: ${BLUE}./session19_final_cleanup.sh${NC}"
echo -e "=================================================="
