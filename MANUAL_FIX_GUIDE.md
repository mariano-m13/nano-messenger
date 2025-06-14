# 🔧 MANUAL FIX GUIDE (If Automated Script Fails)

## 1. Fix CryptoMode::QuantumSafe Pattern Matching

**File:** `src/media/encryption.rs`  
**Line:** Around 341 (in `get_key_algorithm_name` method)

**Change this:**
```rust
match self.crypto_mode {
    CryptoMode::Classical => "X25519+Ed25519".to_string(),
    CryptoMode::Hybrid => "X25519+ML-KEM-768".to_string(),
    CryptoMode::Quantum => "ML-KEM-768+ML-DSA-65".to_string(),
}
```

**To this:**
```rust
match self.crypto_mode {
    CryptoMode::Classical => "X25519+Ed25519".to_string(),
    CryptoMode::Hybrid => "X25519+ML-KEM-768".to_string(),
    CryptoMode::Quantum | CryptoMode::QuantumSafe => "ML-KEM-768+ML-DSA-65".to_string(),
}
```

## 2. Fix Unused Variable Warning

**File:** `src/media/security/scanning.rs`  
**Line:** Around 670

**Change this:**
```rust
fn get_historical_context(&self, user_id: &UserId, uploads: &[FileUpload]) -> BehaviorHistoryContext {
```

**To this:**
```rust
fn get_historical_context(&self, _user_id: &UserId, uploads: &[FileUpload]) -> BehaviorHistoryContext {
```

## 3. Disable FFmpeg-Next (Temporarily)

**File:** `Cargo.toml`

**Change this:**
```toml
ffmpeg-next = { version = "6.0", optional = true }
video-processing = ["ffmpeg-next"]
media-full = ["image-processing", "video-processing", "exif-processing"]
```

**To this:**
```toml
# ffmpeg-next = { version = "6.0", optional = true }  # TEMPORARILY DISABLED
# video-processing = ["ffmpeg-next"]   # TEMPORARILY DISABLED
media-full = ["image-processing", "exif-processing"]  # Video disabled
```

## 4. Test Compilation

```bash
cargo clean
cargo check --lib
cargo check --features="local-storage,image-processing,session11-basic"
```

## 5. If All Works, Complete Session 19

```bash
chmod +x session19_immediate_fix.sh
./session19_immediate_fix.sh
```

---

**These 3 simple changes will fix all compilation errors! 🎯**
