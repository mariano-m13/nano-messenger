#!/bin/bash

echo "🧪 Running comprehensive test of all media transfer functionality..."
cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running ALL media::transfer::tests..."
cargo test media::transfer::tests --lib

echo ""
echo "🎯 Expected: ALL 6 tests should now pass!"
echo "   ✅ test_file_upload_structure"
echo "   ✅ test_mime_type_detection" 
echo "   ✅ test_file_upload_progress"
echo "   ✅ test_transfer_manager_health_check"
echo "   ✅ test_transfer_statistics"
echo "   ✅ test_transfer_progress_calculations"
