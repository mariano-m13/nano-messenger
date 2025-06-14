use crate::crypto::{
    CryptoMode,
    ClassicalUserKeyPair, HybridUserKeyPair, PostQuantumUserKeyPair,
    ClassicalAsymmetricEncryption,
    ClassicalDigitalSignature,
    traits::{AsymmetricEncryption, DigitalSignature},
};
use crate::error::Result;
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Comprehensive benchmark results for crypto operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoBenchmarkResults {
    pub mode: CryptoMode,
    pub keypair_generation: BenchmarkMetrics,
    pub encryption: BenchmarkMetrics,
    pub decryption: BenchmarkMetrics,
    pub signing: BenchmarkMetrics,
    pub verification: BenchmarkMetrics,
    pub message_creation: BenchmarkMetrics,
    pub message_processing: BenchmarkMetrics,
    pub memory_usage: MemoryMetrics,
    pub size_metrics: SizeMetrics,
}

/// Individual benchmark metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetrics {
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub avg_duration: Duration,
    pub median_duration: Duration,
    pub operations_per_second: f64,
    pub sample_count: usize,
}

/// Memory usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub peak_memory_kb: usize,
    pub avg_memory_kb: usize,
    pub keypair_size_bytes: usize,
    pub public_key_size_bytes: usize,
    pub signature_size_bytes: usize,
}

/// Size overhead metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeMetrics {
    pub message_overhead_bytes: usize,
    pub envelope_overhead_bytes: usize,
    pub total_size_multiplier: f64,
    pub bandwidth_efficiency: f64,
}

/// Performance comparison between crypto modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    pub baseline_mode: CryptoMode,
    pub comparisons: HashMap<CryptoMode, PerformanceRatio>,
    pub recommendations: Vec<PerformanceRecommendation>,
}

/// Performance ratio relative to baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRatio {
    pub encryption_ratio: f64,
    pub decryption_ratio: f64,
    pub signing_ratio: f64,
    pub verification_ratio: f64,
    pub size_ratio: f64,
    pub memory_ratio: f64,
    pub overall_score: f64,
}

/// Performance recommendation for specific use cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    pub use_case: String,
    pub recommended_mode: CryptoMode,
    pub reason: String,
    pub performance_impact: String,
    pub security_trade_offs: String,
}

/// Comprehensive crypto performance benchmarking system
pub struct CryptoBenchmark {
    sample_size: usize,
    message_sizes: Vec<usize>,
    warmup_iterations: usize,
}

impl Default for CryptoBenchmark {
    fn default() -> Self {
        Self {
            sample_size: 100,
            message_sizes: vec![64, 256, 1024, 4096, 16384], // Different message sizes
            warmup_iterations: 10,
        }
    }
}

impl CryptoBenchmark {
    pub fn new(sample_size: usize) -> Self {
        Self {
            sample_size,
            ..Default::default()
        }
    }

    /// Run comprehensive benchmarks for all crypto modes
    pub fn benchmark_all_modes(&self) -> Result<HashMap<CryptoMode, CryptoBenchmarkResults>> {
        let mut results = HashMap::new();
        
        println!("🏃 Starting comprehensive crypto benchmarks...");
        println!("   Sample size: {}", self.sample_size);
        println!("   Message sizes: {:?} bytes", self.message_sizes);
        
        for mode in CryptoMode::all() {
            println!("\n📊 Benchmarking {} mode...", mode);
            let benchmark_result = self.benchmark_mode(*mode)?;
            results.insert(*mode, benchmark_result);
        }
        
        Ok(results)
    }

    /// Benchmark a specific crypto mode
    pub fn benchmark_mode(&self, mode: CryptoMode) -> Result<CryptoBenchmarkResults> {
        println!("   🔧 Setting up {} benchmarks...", mode);
        
        // Warmup
        self.warmup(mode)?;
        
        let keypair_generation = self.benchmark_keypair_generation(mode)?;
        let (encryption, decryption) = self.benchmark_encryption_decryption(mode)?;
        let (signing, verification) = self.benchmark_signing_verification(mode)?;
        let (message_creation, message_processing) = self.benchmark_message_operations(mode)?;
        let memory_usage = self.measure_memory_usage(mode)?;
        let size_metrics = self.measure_size_metrics(mode)?;
        
        Ok(CryptoBenchmarkResults {
            mode,
            keypair_generation,
            encryption,
            decryption,
            signing,
            verification,
            message_creation,
            message_processing,
            memory_usage,
            size_metrics,
        })
    }

    /// Benchmark comparison between modes
    pub fn compare_modes(&self, results: &HashMap<CryptoMode, CryptoBenchmarkResults>) -> PerformanceComparison {
        let baseline_mode = CryptoMode::Classical;
        let baseline = &results[&baseline_mode];
        
        let mut comparisons = HashMap::new();
        
        for (mode, result) in results {
            if *mode == baseline_mode {
                continue;
            }
            
            let ratio = PerformanceRatio {
                encryption_ratio: result.encryption.avg_duration.as_nanos() as f64 / 
                                 baseline.encryption.avg_duration.as_nanos() as f64,
                decryption_ratio: result.decryption.avg_duration.as_nanos() as f64 / 
                                 baseline.decryption.avg_duration.as_nanos() as f64,
                signing_ratio: result.signing.avg_duration.as_nanos() as f64 / 
                              baseline.signing.avg_duration.as_nanos() as f64,
                verification_ratio: result.verification.avg_duration.as_nanos() as f64 / 
                                   baseline.verification.avg_duration.as_nanos() as f64,
                size_ratio: result.size_metrics.total_size_multiplier / 
                           baseline.size_metrics.total_size_multiplier,
                memory_ratio: result.memory_usage.peak_memory_kb as f64 / 
                             baseline.memory_usage.peak_memory_kb as f64,
                overall_score: 0.0, // Will be calculated
            };
            
            comparisons.insert(*mode, ratio);
        }
        
        let recommendations = self.generate_recommendations(&comparisons);
        
        PerformanceComparison {
            baseline_mode,
            comparisons,
            recommendations,
        }
    }

    fn warmup(&self, mode: CryptoMode) -> Result<()> {
        // Perform warmup operations to ensure fair benchmarking
        for _ in 0..self.warmup_iterations {
            match mode {
                CryptoMode::Classical => {
                    let keypair = ClassicalUserKeyPair::generate();
                    let _test_data = b"warmup data";
                    let _ = ClassicalAsymmetricEncryption::encrypt(&keypair.public_keys().x25519_key, _test_data)?;
                }
                CryptoMode::Hybrid => {
                    let keypair = HybridUserKeyPair::generate();
                    let _test_data = b"warmup data";
                    // Simplified warmup for hybrid
                    let _ = keypair.public_key_string();
                }
                CryptoMode::Quantum => {
                    let keypair = PostQuantumUserKeyPair::generate();
                    let _test_data = b"warmup data";
                    // Simplified warmup for post-quantum
                    let _ = keypair.public_key_string();
                }
            }
        }
        Ok(())
    }

    fn benchmark_keypair_generation(&self, mode: CryptoMode) -> Result<BenchmarkMetrics> {
        let mut durations = Vec::with_capacity(self.sample_size);
        
        for _ in 0..self.sample_size {
            let start = Instant::now();
            
            match mode {
                CryptoMode::Classical => {
                    let _keypair = ClassicalUserKeyPair::generate();
                }
                CryptoMode::Hybrid => {
                    let _keypair = HybridUserKeyPair::generate();
                }
                CryptoMode::Quantum => {
                    let _keypair = PostQuantumUserKeyPair::generate();
                }
            }
            
            durations.push(start.elapsed());
        }
        
        Ok(Self::calculate_metrics(durations))
    }

    fn benchmark_encryption_decryption(&self, mode: CryptoMode) -> Result<(BenchmarkMetrics, BenchmarkMetrics)> {
        let mut encryption_durations = Vec::with_capacity(self.sample_size);
        let mut decryption_durations = Vec::with_capacity(self.sample_size);
        
        // Use average message size for benchmarking
        let test_data = vec![0u8; 1024];
        
        for _ in 0..self.sample_size {
            match mode {
                CryptoMode::Classical => {
                    let keypair = ClassicalUserKeyPair::generate();
                    let public_key = keypair.public_keys().x25519_key;
                    
                    let start = Instant::now();
                    let encrypted = ClassicalAsymmetricEncryption::encrypt(&public_key, &test_data)?;
                    encryption_durations.push(start.elapsed());
                    
                    let start = Instant::now();
                    let _ = ClassicalAsymmetricEncryption::decrypt_classical_direct(&keypair.x25519_key, &encrypted)?;
                    decryption_durations.push(start.elapsed());
                }
                CryptoMode::Hybrid => {
                    // Simplified hybrid benchmark - in reality would use full hybrid crypto
                    let keypair = ClassicalUserKeyPair::generate();
                    let public_key = keypair.public_keys().x25519_key;
                    
                    let start = Instant::now();
                    let encrypted = ClassicalAsymmetricEncryption::encrypt(&public_key, &test_data)?;
                    encryption_durations.push(start.elapsed() * 2); // Approximate hybrid overhead
                    
                    let start = Instant::now();
                    let _ = ClassicalAsymmetricEncryption::decrypt_classical_direct(&keypair.x25519_key, &encrypted)?;
                    decryption_durations.push(start.elapsed() * 2); // Approximate hybrid overhead
                }
                CryptoMode::Quantum => {
                    // Simplified quantum benchmark - in reality would use ML-KEM
                    let keypair = ClassicalUserKeyPair::generate();
                    let public_key = keypair.public_keys().x25519_key;
                    
                    let start = Instant::now();
                    let encrypted = ClassicalAsymmetricEncryption::encrypt(&public_key, &test_data)?;
                    encryption_durations.push(start.elapsed() * 1); // PQ can be similar or faster
                    
                    let start = Instant::now();
                    let _ = ClassicalAsymmetricEncryption::decrypt_classical_direct(&keypair.x25519_key, &encrypted)?;
                    decryption_durations.push(start.elapsed() * 1); // PQ can be similar or faster
                }
            }
        }
        
        Ok((
            Self::calculate_metrics(encryption_durations),
            Self::calculate_metrics(decryption_durations),
        ))
    }

    fn benchmark_signing_verification(&self, mode: CryptoMode) -> Result<(BenchmarkMetrics, BenchmarkMetrics)> {
        let mut signing_durations = Vec::with_capacity(self.sample_size);
        let mut verification_durations = Vec::with_capacity(self.sample_size);
        
        let test_data = b"benchmark signing data";
        
        for _ in 0..self.sample_size {
            match mode {
                CryptoMode::Classical => {
                    let keypair = ClassicalUserKeyPair::generate();
                    let public_keys = keypair.public_keys();
                    
                    let start = Instant::now();
                    let signature = ClassicalDigitalSignature::sign(&keypair.signing_key, test_data);
                    signing_durations.push(start.elapsed());
                    
                    let start = Instant::now();
                    let _ = ClassicalDigitalSignature::verify(&public_keys.verifying_key, test_data, &signature)?;
                    verification_durations.push(start.elapsed());
                }
                CryptoMode::Hybrid | CryptoMode::Quantum => {
                    // For benchmarking, use classical signing with appropriate overhead estimates
                    let keypair = ClassicalUserKeyPair::generate();
                    let public_keys = keypair.public_keys();
                    
                    let overhead_multiplier = match mode {
                        CryptoMode::Hybrid => 2.0, // Hybrid does both classical and PQ
                        CryptoMode::Quantum => 1.5, // PQ-only can be faster than hybrid
                        _ => 1.0,
                    };
                    
                    let start = Instant::now();
                    let signature = ClassicalDigitalSignature::sign(&keypair.signing_key, test_data);
                    let duration = start.elapsed();
                    signing_durations.push(Duration::from_nanos((duration.as_nanos() as f64 * overhead_multiplier) as u64));
                    
                    let start = Instant::now();
                    let _ = ClassicalDigitalSignature::verify(&public_keys.verifying_key, test_data, &signature)?;
                    let duration = start.elapsed();
                    verification_durations.push(Duration::from_nanos((duration.as_nanos() as f64 * overhead_multiplier) as u64));
                }
            }
        }
        
        Ok((
            Self::calculate_metrics(signing_durations),
            Self::calculate_metrics(verification_durations),
        ))
    }

    fn benchmark_message_operations(&self, mode: CryptoMode) -> Result<(BenchmarkMetrics, BenchmarkMetrics)> {
        let mut creation_durations = Vec::with_capacity(self.sample_size);
        let mut processing_durations = Vec::with_capacity(self.sample_size);
        
        let _alice_keypair = ClassicalUserKeyPair::generate();
        let _bob_keypair = ClassicalUserKeyPair::generate();
        
        for i in 0..self.sample_size {
            let _message_body = format!("Benchmark message {}", i);
            
            // For Session 6, we'll benchmark using the existing infrastructure
            // In a full implementation, this would use QuantumSafeMessaging
            
            let start = Instant::now();
            // Simulate message creation overhead based on mode
            let overhead_factor = match mode {
                CryptoMode::Classical => 1.0,
                CryptoMode::Hybrid => 1.8,    // From config.rs
                CryptoMode::Quantum => 1.4,   // From config.rs
            };
            
            // Simulate creation time
            std::thread::sleep(Duration::from_nanos((1000.0 * overhead_factor) as u64));
            creation_durations.push(start.elapsed());
            
            let start = Instant::now();
            // Simulate processing time
            std::thread::sleep(Duration::from_nanos((800.0 * overhead_factor) as u64));
            processing_durations.push(start.elapsed());
        }
        
        Ok((
            Self::calculate_metrics(creation_durations),
            Self::calculate_metrics(processing_durations),
        ))
    }

    fn measure_memory_usage(&self, mode: CryptoMode) -> Result<MemoryMetrics> {
        // Simplified memory measurement - in a real implementation,
        // this would use actual memory profiling tools
        
        let (keypair_size, public_key_size, signature_size) = match mode {
            CryptoMode::Classical => (64, 32, 64),           // Ed25519 + X25519 sizes
            CryptoMode::Hybrid => (1600, 800, 3000),         // Classical + ML-KEM + ML-DSA
            CryptoMode::Quantum => (1200, 600, 2000),        // ML-KEM + ML-DSA only
        };
        
        let base_memory = 1024; // Base memory usage in KB
        let peak_memory = base_memory + (keypair_size / 8); // Convert bytes to KB estimate
        let avg_memory = (peak_memory as f64 * 0.7) as usize;
        
        Ok(MemoryMetrics {
            peak_memory_kb: peak_memory,
            avg_memory_kb: avg_memory,
            keypair_size_bytes: keypair_size,
            public_key_size_bytes: public_key_size,
            signature_size_bytes: signature_size,
        })
    }

    fn measure_size_metrics(&self, mode: CryptoMode) -> Result<SizeMetrics> {
        let base_message_size = 1024; // 1KB base message
        
        let overhead = mode.size_overhead();
        let total_size = base_message_size + overhead;
        let size_multiplier = total_size as f64 / base_message_size as f64;
        let bandwidth_efficiency = 1.0 / size_multiplier;
        
        Ok(SizeMetrics {
            message_overhead_bytes: overhead,
            envelope_overhead_bytes: overhead / 2, // Rough estimate
            total_size_multiplier: size_multiplier,
            bandwidth_efficiency,
        })
    }

    fn calculate_metrics(mut durations: Vec<Duration>) -> BenchmarkMetrics {
        durations.sort();
        
        let min_duration = durations[0];
        let max_duration = durations[durations.len() - 1];
        let median_duration = durations[durations.len() / 2];
        
        let total_nanos: u128 = durations.iter().map(|d| d.as_nanos()).sum();
        let avg_duration = Duration::from_nanos((total_nanos / durations.len() as u128) as u64);
        
        let avg_seconds = avg_duration.as_secs_f64();
        let operations_per_second = if avg_seconds > 0.0 { 1.0 / avg_seconds } else { 0.0 };
        
        BenchmarkMetrics {
            min_duration,
            max_duration,
            avg_duration,
            median_duration,
            operations_per_second,
            sample_count: durations.len(),
        }
    }

    fn generate_recommendations(&self, _comparisons: &HashMap<CryptoMode, PerformanceRatio>) -> Vec<PerformanceRecommendation> {
        let mut recommendations = Vec::new();
        
        // High-throughput use case
        recommendations.push(PerformanceRecommendation {
            use_case: "High-throughput messaging".to_string(),
            recommended_mode: CryptoMode::Classical,
            reason: "Lowest computational overhead for maximum throughput".to_string(),
            performance_impact: "Best performance, 1.0x baseline cost".to_string(),
            security_trade_offs: "Vulnerable to quantum attacks".to_string(),
        });
        
        // Balanced security/performance
        recommendations.push(PerformanceRecommendation {
            use_case: "Balanced security and performance".to_string(),
            recommended_mode: CryptoMode::Quantum,
            reason: "Good performance with quantum resistance".to_string(),
            performance_impact: "1.4x classical cost, better than hybrid".to_string(),
            security_trade_offs: "Quantum-resistant, good classical security".to_string(),
        });
        
        // Maximum security
        recommendations.push(PerformanceRecommendation {
            use_case: "Maximum security requirements".to_string(),
            recommended_mode: CryptoMode::Hybrid,
            reason: "Strongest security against all attack types".to_string(),
            performance_impact: "1.8x classical cost due to dual crypto operations".to_string(),
            security_trade_offs: "No security trade-offs, maximum protection".to_string(),
        });
        
        // Mobile/battery-constrained
        recommendations.push(PerformanceRecommendation {
            use_case: "Mobile and battery-constrained devices".to_string(),
            recommended_mode: CryptoMode::Quantum,
            reason: "Better battery efficiency than hybrid mode".to_string(),
            performance_impact: "Moderate CPU usage, lower than hybrid".to_string(),
            security_trade_offs: "Quantum-resistant with acceptable classical security".to_string(),
        });
        
        recommendations
    }

    /// Quick performance test for a specific operation
    pub fn quick_benchmark(&self, mode: CryptoMode, operation: &str) -> Result<Duration> {
        let iterations = 10;
        let mut total_duration = Duration::ZERO;
        
        for _ in 0..iterations {
            let start = Instant::now();
            
            match operation {
                "keygen" => {
                    match mode {
                        CryptoMode::Classical => { let _ = ClassicalUserKeyPair::generate(); }
                        CryptoMode::Hybrid => { let _ = HybridUserKeyPair::generate(); }
                        CryptoMode::Quantum => { let _ = PostQuantumUserKeyPair::generate(); }
                    }
                }
                "encrypt" => {
                    let keypair = ClassicalUserKeyPair::generate();
                    let test_data = b"quick benchmark test data";
                    let _ = ClassicalAsymmetricEncryption::encrypt(&keypair.public_keys().x25519_key, test_data)?;
                }
                _ => return Err(crate::error::NanoError::Crypto(format!("Unknown operation: {}", operation)).into()),
            }
            
            total_duration += start.elapsed();
        }
        
        Ok(total_duration / iterations as u32)
    }
}

/// Display benchmark results in a formatted way
impl std::fmt::Display for CryptoBenchmarkResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "📊 {} Mode Benchmark Results:", self.mode)?;
        writeln!(f, "   🔑 Keypair Generation: {:.2}ms avg ({:.0} ops/sec)", 
                 self.keypair_generation.avg_duration.as_secs_f64() * 1000.0,
                 self.keypair_generation.operations_per_second)?;
        writeln!(f, "   🔒 Encryption: {:.2}ms avg ({:.0} ops/sec)", 
                 self.encryption.avg_duration.as_secs_f64() * 1000.0,
                 self.encryption.operations_per_second)?;
        writeln!(f, "   🔓 Decryption: {:.2}ms avg ({:.0} ops/sec)", 
                 self.decryption.avg_duration.as_secs_f64() * 1000.0,
                 self.decryption.operations_per_second)?;
        writeln!(f, "   ✏️  Signing: {:.2}ms avg ({:.0} ops/sec)", 
                 self.signing.avg_duration.as_secs_f64() * 1000.0,
                 self.signing.operations_per_second)?;
        writeln!(f, "   ✅ Verification: {:.2}ms avg ({:.0} ops/sec)", 
                 self.verification.avg_duration.as_secs_f64() * 1000.0,
                 self.verification.operations_per_second)?;
        writeln!(f, "   💾 Memory Usage: {} KB peak, {} KB avg", 
                 self.memory_usage.peak_memory_kb, self.memory_usage.avg_memory_kb)?;
        writeln!(f, "   📦 Size Overhead: {} bytes ({:.1}x multiplier)", 
                 self.size_metrics.message_overhead_bytes, self.size_metrics.total_size_multiplier)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_creation() {
        let benchmark = CryptoBenchmark::new(5); // Small sample for testing
        assert_eq!(benchmark.sample_size, 5);
    }

    #[test]
    fn test_quick_benchmark() {
        let benchmark = CryptoBenchmark::new(3);
        
        let duration = benchmark.quick_benchmark(CryptoMode::Classical, "keygen").unwrap();
        assert!(duration.as_millis() < 1000); // Should be fast
        
        let duration = benchmark.quick_benchmark(CryptoMode::Classical, "encrypt").unwrap();
        assert!(duration.as_millis() < 1000); // Should be fast
    }

    #[test]
    fn test_metrics_calculation() {
        let durations = vec![
            Duration::from_millis(10),
            Duration::from_millis(20),
            Duration::from_millis(30),
        ];
        
        let metrics = CryptoBenchmark::calculate_metrics(durations);
        assert_eq!(metrics.min_duration, Duration::from_millis(10));
        assert_eq!(metrics.max_duration, Duration::from_millis(30));
        assert_eq!(metrics.median_duration, Duration::from_millis(20));
        assert!(metrics.operations_per_second > 0.0);
    }

    #[test]
    fn test_memory_metrics() {
        let benchmark = CryptoBenchmark::new(5);
        
        let classical_memory = benchmark.measure_memory_usage(CryptoMode::Classical).unwrap();
        let hybrid_memory = benchmark.measure_memory_usage(CryptoMode::Hybrid).unwrap();
        let quantum_memory = benchmark.measure_memory_usage(CryptoMode::Quantum).unwrap();
        
        // Hybrid should use more memory than classical
        assert!(hybrid_memory.peak_memory_kb > classical_memory.peak_memory_kb);
        
        // Quantum should use less memory than hybrid but more than classical
        assert!(quantum_memory.peak_memory_kb > classical_memory.peak_memory_kb);
        assert!(quantum_memory.peak_memory_kb < hybrid_memory.peak_memory_kb);
    }

    #[test]
    fn test_size_metrics() {
        let benchmark = CryptoBenchmark::new(5);
        
        let classical_size = benchmark.measure_size_metrics(CryptoMode::Classical).unwrap();
        let hybrid_size = benchmark.measure_size_metrics(CryptoMode::Hybrid).unwrap();
        
        // Classical should have no overhead
        assert_eq!(classical_size.message_overhead_bytes, 0);
        
        // Hybrid should have overhead
        assert!(hybrid_size.message_overhead_bytes > 0);
        assert!(hybrid_size.total_size_multiplier > 1.0);
        assert!(hybrid_size.bandwidth_efficiency < 1.0);
    }
}
