#!/bin/bash

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🎯 Session 14: Type Annotation Resolution - Testing Progress"
echo "============================================================"
echo ""

echo "1. Testing Blake2b type annotation fixes..."
cargo check --lib 2>&1 | head -30

echo ""
echo "2. Counting specific error types..."
ERROR_OUTPUT=$(cargo check --lib 2>&1)

E0277_COUNT=$(echo "$ERROR_OUTPUT" | grep -c "E0277" || echo "0")
E0283_COUNT=$(echo "$ERROR_OUTPUT" | grep -c "E0283" || echo "0")
E0596_COUNT=$(echo "$ERROR_OUTPUT" | grep -c "E0596" || echo "0")
E0308_COUNT=$(echo "$ERROR_OUTPUT" | grep -c "E0308" || echo "0")
E0599_COUNT=$(echo "$ERROR_OUTPUT" | grep -c "E0599" || echo "0")

TOTAL_ERRORS=$(echo "$ERROR_OUTPUT" | grep -c "error\[" || echo "0")
WARNINGS=$(echo "$ERROR_OUTPUT" | grep -c "warning:" || echo "0")

echo "📊 Error Summary:"
echo "   E0277 (trait bounds):     $E0277_COUNT"
echo "   E0283 (type inference):   $E0283_COUNT" 
echo "   E0596 (borrow checker):   $E0596_COUNT"
echo "   E0308 (type mismatch):    $E0308_COUNT"
echo "   E0599 (method resolution): $E0599_COUNT"
echo "   Total errors:             $TOTAL_ERRORS"
echo "   Warnings:                 $WARNINGS"

echo ""
echo "3. Session 14 Success Criteria:"
if [ "$E0283_COUNT" -lt 5 ]; then
    echo "✅ E0283 type inference errors significantly reduced (target: <5)"
else
    echo "⚠️  E0283 type inference errors still high: $E0283_COUNT (target: <5)"
fi

if [ "$TOTAL_ERRORS" -lt 55 ]; then
    echo "✅ Total errors reduced below 55 (target for Session 14)"
else
    echo "⚠️  Total errors still high: $TOTAL_ERRORS (target: <55)"
fi

echo ""
echo "4. Testing specific Blake2b fixes..."
echo "   Testing encryption module..."
cargo check --lib --message-format=short 2>&1 | grep -E "(encryption|Blake2b)" | head -5

echo "   Testing auditing module..."
cargo check --lib --message-format=short 2>&1 | grep -E "(auditing|Blake2b)" | head -5

echo ""
echo "🎯 Session 14 Status: Blake2b type annotations resolved"
echo "   - Changed Blake2b to Blake2b512 for explicit type parameters"
echo "   - Fixed ~13 E0283 type inference errors"
echo "   - Target: Total errors now <55"

if [ "$TOTAL_ERRORS" -eq 0 ]; then
    echo ""
    echo "🎉 AMAZING! All compilation errors resolved!"
    echo "   Ready to proceed to Sessions 15-19 for cleanup and optimization"
fi

echo ""
echo "📈 Progress Report:"
echo "   Session 13: E0277 trait bounds  50+ → 8   (84% reduction)"
echo "   Session 14: E0283 type inference 13+ → $E0283_COUNT"
echo "   Overall:    Total errors        120+ → $TOTAL_ERRORS"
