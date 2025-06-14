#!/bin/bash

echo "Testing the 3 specific test fixes..."
cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running the specific failing tests..."
cargo test media::transfer::tests::test_file_upload_progress --lib
echo ""
cargo test media::transfer::tests::test_mime_type_detection --lib  
echo ""
cargo test media::transfer::tests::test_transfer_manager_health_check --lib

echo ""
echo "If all 3 tests pass, all issues have been resolved!"
