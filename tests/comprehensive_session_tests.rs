//! Comprehensive Session Integration Tests
//! 
//! This module provides programmatic testing of all session validations
//! for the nano-messenger quantum-resistant protocol.

use std::process::Command;
use std::time::{Duration, Instant};

/// Test result for individual session validation
#[derive(Debug, Clone)]
pub struct SessionTestResult {
    pub session_name: String,
    pub passed: bool,
    pub duration: Duration,
    pub output: String,
    pub error: Option<String>,
}

/// Comprehensive test results for all sessions
#[derive(Debug)]
pub struct ComprehensiveTestResults {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub session_results: Vec<SessionTestResult>,
    pub total_duration: Duration,
}

impl ComprehensiveTestResults {
    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.failed_tests == 0
    }
    
    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0
        }
    }
    
    /// Print a summary report
    pub fn print_summary(&self) {
        println!("\n🧪 COMPREHENSIVE SESSION TEST RESULTS");
        println!("=====================================");
        println!("Total Tests: {}", self.total_tests);
        println!("Passed: {}", self.passed_tests);
        println!("Failed: {}", self.failed_tests);
        println!("Success Rate: {:.1}%", self.success_rate());
        println!("Total Duration: {:.2}s", self.total_duration.as_secs_f64());
        
        println!("\nDetailed Results:");
        for result in &self.session_results {
            let status = if result.passed { "✅ PASSED" } else { "❌ FAILED" };
            println!("  {} - {} ({:.2}s)", result.session_name, status, result.duration.as_secs_f64());
            
            if !result.passed && result.error.is_some() {
                println!("    Error: {}", result.error.as_ref().unwrap());
            }
        }
        
        if self.all_passed() {
            println!("\n🎉 ALL TESTS PASSED! Protocol is fully validated!");
        } else {
            println!("\n⚠️  Some tests failed. Review errors before deployment.");
        }
    }
}

/// Run a single session validation example
fn run_session_validation(session_name: &str) -> SessionTestResult {
    let example_name = format!("{}_validation", session_name.to_lowercase().replace(' ', '_'));
    
    println!("🔍 Testing: {}", session_name);
    
    let start_time = Instant::now();
    
    let output = Command::new("cargo")
        .args(&["run", "--example", &example_name])
        .output();
    
    let duration = start_time.elapsed();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined_output = format!("{}\n{}", stdout, stderr);
            
            let passed = output.status.success() && 
                        (stdout.contains("COMPLETE") || 
                         stdout.contains("SUCCESS") || 
                         stdout.contains("✅"));
            
            SessionTestResult {
                session_name: session_name.to_string(),
                passed,
                duration,
                output: combined_output,
                error: if passed { None } else { Some(stderr.to_string()) },
            }
        }
        Err(e) => {
            SessionTestResult {
                session_name: session_name.to_string(),
                passed: false,
                duration,
                output: String::new(),
                error: Some(format!("Failed to execute cargo command: {}", e)),
            }
        }
    }
}

/// Run comprehensive validation of all sessions
pub fn run_comprehensive_session_tests() -> ComprehensiveTestResults {
    let start_time = Instant::now();
    
    // Define all sessions to test
    let sessions = vec![
        "Session 1",
        "Session 2", 
        "Session 3",
        "Session 4",
        "Session 5",
        "Session 6",
        "Session 7",
    ];
    
    println!("🧪 Starting comprehensive session validation...");
    println!("Testing {} sessions...\n", sessions.len());
    
    let mut session_results = Vec::new();
    let mut passed_count = 0;
    
    // Test each session
    for session in &sessions {
        let result = run_session_validation(session);
        
        if result.passed {
            passed_count += 1;
            println!("   ✅ {} PASSED ({:.2}s)", session, result.duration.as_secs_f64());
        } else {
            println!("   ❌ {} FAILED ({:.2}s)", session, result.duration.as_secs_f64());
        }
        
        session_results.push(result);
    }
    
    let total_duration = start_time.elapsed();
    
    ComprehensiveTestResults {
        total_tests: sessions.len(),
        passed_tests: passed_count,
        failed_tests: sessions.len() - passed_count,
        session_results,
        total_duration,
    }
}

/// Test that verifies compilation of all examples
pub fn test_compilation() -> bool {
    println!("🔨 Testing compilation of all examples...");
    
    let output = Command::new("cargo")
        .args(&["check", "--examples"])
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                println!("   ✅ All examples compile successfully");
                true
            } else {
                println!("   ❌ Compilation failed");
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("   Error: {}", stderr);
                false
            }
        }
        Err(e) => {
            println!("   ❌ Failed to run cargo check: {}", e);
            false
        }
    }
}

/// Integration test that runs the comprehensive session validation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_sessions_comprehensive() {
        // First ensure everything compiles
        assert!(test_compilation(), "All examples must compile before testing");
        
        // Run comprehensive session tests
        let results = run_comprehensive_session_tests();
        results.print_summary();
        
        // Assert that all tests passed
        assert!(
            results.all_passed(),
            "All session validations must pass. Failed tests: {}/{}",
            results.failed_tests,
            results.total_tests
        );
        
        // Verify specific critical sessions
        let session7_result = results.session_results
            .iter()
            .find(|r| r.session_name == "Session 7");
        
        assert!(
            session7_result.is_some() && session7_result.unwrap().passed,
            "Session 7 (Security Validation) must pass for production readiness"
        );
    }
    
    #[test]
    fn test_compilation_only() {
        // Quick test that just verifies everything compiles
        assert!(test_compilation(), "All code must compile without errors");
    }
    
    #[test] 
    fn test_critical_sessions() {
        // Test only the most critical sessions for quick validation
        let critical_sessions = vec!["Session 7"]; // Security validation is most critical
        
        println!("🚀 Testing critical sessions for quick validation...");
        
        for session in critical_sessions {
            let result = run_session_validation(session);
            assert!(
                result.passed,
                "Critical session '{}' failed: {:?}",
                session,
                result.error
            );
        }
    }
    
    #[test]
    fn test_session_performance() {
        // Test that sessions complete within reasonable time
        println!("⏱️  Testing session performance...");
        
        let result = run_session_validation("Session 7");
        
        // Session 7 should complete within 30 seconds
        assert!(
            result.duration < Duration::from_secs(30),
            "Session 7 took too long: {:.2}s (should be < 30s)",
            result.duration.as_secs_f64()
        );
        
        assert!(
            result.passed,
            "Session 7 must pass for security validation"
        );
    }
}

#[cfg(test)]
mod benchmark_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    #[ignore] // Use `cargo test -- --ignored` to run benchmarks
    fn benchmark_all_sessions() {
        println!("📊 Benchmarking all session validations...");
        
        // Run each session multiple times to get average performance
        let sessions = vec!["Session 1", "Session 7"]; // Test subset for benchmarking
        
        for session in sessions {
            let mut durations = Vec::new();
            
            for run in 1..=3 {
                println!("   Run {}/3 for {}", run, session);
                let result = run_session_validation(session);
                assert!(result.passed, "Session {} failed during benchmarking", session);
                durations.push(result.duration);
            }
            
            let avg_duration: Duration = durations.iter().sum::<Duration>() / durations.len() as u32;
            let min_duration = durations.iter().min().unwrap();
            let max_duration = durations.iter().max().unwrap();
            
            println!("📈 {} Performance:", session);
            println!("   Average: {:.2}s", avg_duration.as_secs_f64());
            println!("   Min: {:.2}s", min_duration.as_secs_f64());
            println!("   Max: {:.2}s", max_duration.as_secs_f64());
        }
    }
}
