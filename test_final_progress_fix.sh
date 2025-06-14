#!/bin/bash

echo "Testing the final progress calculation fix..."
cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running the progress test with 30-byte content..."
cargo test media::transfer::tests::test_file_upload_progress --lib

echo ""
echo "If this passes, all test issues are completely resolved!"
