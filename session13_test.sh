#!/bin/bash

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🎯 Session 13: Trait Implementation Foundation - Testing Progress"
echo "================================================================="
echo ""

echo "1. Testing basic library compilation..."
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
echo "   E0596 (borrow checker):   $E0596COUNT"
echo "   E0308 (type mismatch):    $E0308_COUNT"
echo "   E0599 (method resolution): $E0599_COUNT"
echo "   Total errors:             $TOTAL_ERRORS"
echo "   Warnings:                 $WARNINGS"

echo ""
echo "3. Session 13 Success Criteria:"
if [ "$E0277_COUNT" -lt 10 ]; then
    echo "✅ E0277 trait bound errors significantly reduced (target: <10)"
else
    echo "⚠️  E0277 trait bound errors still high: $E0277_COUNT (target: <10)"
fi

if [ "$TOTAL_ERRORS" -lt 70 ]; then
    echo "✅ Total errors reduced below 70 (target for Session 13)"
else
    echo "⚠️  Total errors still high: $TOTAL_ERRORS (target: <70)"
fi

echo ""
echo "4. Testing specific compliance modules..."
echo "   Testing HIPAA module..."
cargo check --lib --message-format=short 2>&1 | grep -E "(hipaa|HIPAA)" | head -5

echo "   Testing GDPR module..."
cargo check --lib --message-format=short 2>&1 | grep -E "(gdpr|GDPR)" | head -5

echo "   Testing Auditing module..."
cargo check --lib --message-format=short 2>&1 | grep -E "(auditing|Audit)" | head -5

echo ""
echo "🎯 Session 13 Status: Trait implementations added to key enums in compliance modules"
echo "   - Added Hash, PartialEq, Eq to HashMap key enums"
echo "   - Added Ord where needed for ordering operations" 
echo "   - Target: Resolve ~50 E0277 trait bound errors"

if [ "$TOTAL_ERRORS" -eq 0 ]; then
    echo ""
    echo "🎉 INCREDIBLE! All compilation errors resolved!"
    echo "   Ready to proceed to Session 14 (Type Annotation Resolution)"
fi
