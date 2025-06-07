#!/usr/bin/env python3

"""
Basic test script for nano-messenger functionality
This tests the core crypto and protocol functionality without networking
"""

import json
import subprocess
import tempfile
import os
import shutil

def run_command(cmd, cwd=None):
    """Run a command and return stdout, stderr, returncode"""
    result = subprocess.run(
        cmd, 
        shell=True, 
        capture_output=True, 
        text=True, 
        cwd=cwd
    )
    return result.stdout, result.stderr, result.returncode

def test_build():
    """Test that the project builds successfully"""
    print("🔧 Testing build...")
    stdout, stderr, code = run_command("cargo build", "/Users/mariano/Desktop/Code/nano-messenger")
    
    if code != 0:
        print(f"❌ Build failed: {stderr}")
        return False
    
    print("✅ Build successful")
    return True

def test_crypto_module():
    """Test crypto functionality"""
    print("🔐 Testing crypto module...")
    stdout, stderr, code = run_command("cargo test crypto", "/Users/mariano/Desktop/Code/nano-messenger")
    
    if code != 0:
        print(f"❌ Crypto tests failed: {stderr}")
        return False
    
    print("✅ Crypto tests passed")
    return True

def test_protocol_module():
    """Test protocol functionality"""
    print("📡 Testing protocol module...")
    stdout, stderr, code = run_command("cargo test protocol", "/Users/mariano/Desktop/Code/nano-messenger")
    
    if code != 0:
        print(f"❌ Protocol tests failed: {stderr}")
        return False
    
    print("✅ Protocol tests passed")
    return True

def test_client_init():
    """Test client initialization"""
    print("👤 Testing client initialization...")
    
    # First build the release binary
    stdout, stderr, code = run_command("cargo build --release", "/Users/mariano/Desktop/Code/nano-messenger")
    if code != 0:
        print(f"❌ Failed to build release binary: {stderr}")
        return False
    
    with tempfile.TemporaryDirectory() as temp_dir:
        config_dir = os.path.join(temp_dir, "test-config")
        
        stdout, stderr, code = run_command(
            f"./target/release/nano-client --config-dir {config_dir} init",
            "/Users/mariano/Desktop/Code/nano-messenger"
        )
        
        if code != 0:
            print(f"❌ Client init failed: {stderr}")
            return False
        
        # Check that keys file was created
        keys_file = os.path.join(config_dir, "keys.json")
        if not os.path.exists(keys_file):
            print("❌ Keys file not created")
            return False
        
        # Verify keys file has valid JSON
        try:
            with open(keys_file) as f:
                keys_data = json.load(f)
                
            required_keys = ["signing_key", "x25519_key", "verifying_key", "x25519_public"]
            for key in required_keys:
                if key not in keys_data:
                    print(f"❌ Missing key in keys file: {key}")
                    return False
        except Exception as e:
            print(f"❌ Invalid keys file: {e}")
            return False
    
    print("✅ Client initialization test passed")
    return True

def main():
    print("🧪 Running Nano Messenger Tests")
    print("=" * 40)
    
    tests = [
        test_build,
        test_crypto_module,
        test_protocol_module,
        test_client_init,
    ]
    
    passed = 0
    total = len(tests)
    
    for test in tests:
        if test():
            passed += 1
        print()
    
    print("=" * 40)
    print(f"📊 Test Results: {passed}/{total} tests passed")
    
    if passed == total:
        print("🎉 All tests passed!")
        return 0
    else:
        print("❌ Some tests failed")
        return 1

if __name__ == "__main__":
    exit(main())
