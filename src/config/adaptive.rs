use std::collections::HashMap;
use crate::crypto::CryptoMode;
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};
use std::collections::VecDeque;

/// Configuration for adaptive crypto mode selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveConfig {
    pub enable_bandwidth_adaptation: bool,
    pub enable_battery_adaptation: bool,
    pub enable_latency_adaptation: bool,
    pub enable_cpu_adaptation: bool,
    pub measurement_window_seconds: u64,
    pub min_samples_for_decision: usize,
    pub adaptation_threshold: f64,
    pub fallback_mode: CryptoMode,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            enable_bandwidth_adaptation: true,
            enable_battery_adaptation: true,
            enable_latency_adaptation: true,
            enable_cpu_adaptation: true,
            measurement_window_seconds: 300, // 5 minutes
            min_samples_for_decision: 5,
            adaptation_threshold: 0.15, // 15% change threshold
            fallback_mode: CryptoMode::Classical,
        }
    }
}

/// Current network conditions affecting crypto mode selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConditions {
    pub bandwidth_mbps: f64,
    pub latency_ms: f64,
    pub packet_loss_rate: f64,
    pub connection_stability: ConnectionStability,
    pub is_metered: bool,
    pub signal_strength: SignalStrength,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConnectionStability {
    Stable,
    Unstable,
    VeryUnstable,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SignalStrength {
    Excellent, // > 80%
    Good,      // 60-80%
    Fair,      // 40-60%
    Poor,      // < 40%
}

/// Device constraints that affect crypto performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConstraints {
    pub battery_level_percent: f64,
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub thermal_state: ThermalState,
    pub power_source: PowerSource,
    pub device_class: DeviceClass,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ThermalState {
    Normal,
    Warm,
    Hot,
    Critical,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PowerSource {
    Battery,
    Charging,
    PluggedIn,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeviceClass {
    Mobile,
    Tablet,
    Laptop,
    Desktop,
    Server,
    IoT,
}

/// Recommendation from the adaptive system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveRecommendation {
    pub recommended_mode: CryptoMode,
    pub confidence: f64, // 0.0 to 1.0
    pub reasoning: Vec<String>,
    pub expected_performance_impact: f64, // relative to baseline
    pub expected_battery_impact: f64,     // relative to baseline
    pub expected_bandwidth_impact: f64,   // relative to baseline
    pub valid_for_seconds: u64,
}

/// Historical measurement for trend analysis
#[derive(Debug, Clone)]
struct Measurement {
    timestamp: Instant,
    _network_conditions: NetworkConditions,
    _device_constraints: DeviceConstraints,
    crypto_mode: CryptoMode,
    performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub encryption_time_ms: f64,
    pub decryption_time_ms: f64,
    pub message_size_bytes: usize,
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: f64,
    pub battery_drain_mw: f64,
}

/// Adaptive mode selector that chooses optimal crypto modes based on conditions
pub struct AdaptiveModeSelector {
    config: AdaptiveConfig,
    measurements: VecDeque<Measurement>,
    current_recommendation: Option<AdaptiveRecommendation>,
    last_update: Instant,
}

impl AdaptiveModeSelector {
    pub fn new(config: AdaptiveConfig) -> Self {
        Self {
            config,
            measurements: VecDeque::new(),
            current_recommendation: None,
            last_update: Instant::now(),
        }
    }

    /// Get the recommended crypto mode based on current conditions
    pub fn recommend_mode(
        &mut self,
        network: &NetworkConditions,
        device: &DeviceConstraints,
    ) -> AdaptiveRecommendation {
        // Check if we need to update the recommendation
        if self.should_update_recommendation() {
            self.current_recommendation = Some(self.calculate_recommendation(network, device));
            self.last_update = Instant::now();
        }

        self.current_recommendation
            .clone()
            .unwrap_or_else(|| self.fallback_recommendation())
    }

    /// Record performance metrics for a crypto operation
    pub fn record_performance(
        &mut self,
        network: NetworkConditions,
        device: DeviceConstraints,
        mode: CryptoMode,
        metrics: PerformanceMetrics,
    ) {
        let measurement = Measurement {
            timestamp: Instant::now(),
            _network_conditions: network,
            _device_constraints: device,
            crypto_mode: mode,
            performance_metrics: metrics,
        };

        self.measurements.push_back(measurement);
        self.cleanup_old_measurements();
    }

    /// Get performance trends for analysis
    pub fn get_performance_trends(&self) -> PerformanceTrends {
        let window = Duration::from_secs(self.config.measurement_window_seconds);
        let cutoff = Instant::now() - window;

        let recent_measurements: Vec<&Measurement> = self
            .measurements
            .iter()
            .filter(|m| m.timestamp > cutoff)
            .collect();

        self.analyze_trends(&recent_measurements)
    }

    fn should_update_recommendation(&self) -> bool {
        // Update every 30 seconds or if no recommendation exists
        self.current_recommendation.is_none()
            || self.last_update.elapsed() > Duration::from_secs(30)
    }

    fn calculate_recommendation(
        &self,
        network: &NetworkConditions,
        device: &DeviceConstraints,
    ) -> AdaptiveRecommendation {
        let mut reasoning = Vec::new();
        let mut score_classical = 1.0;
        let mut score_hybrid = 0.8;
        let mut score_quantum = 0.9;

        // Bandwidth considerations
        if self.config.enable_bandwidth_adaptation {
            self.apply_bandwidth_scoring(network, &mut score_classical, &mut score_hybrid, &mut score_quantum, &mut reasoning);
        }

        // Battery considerations  
        if self.config.enable_battery_adaptation {
            self.apply_battery_scoring(device, &mut score_classical, &mut score_hybrid, &mut score_quantum, &mut reasoning);
        }

        // Latency considerations
        if self.config.enable_latency_adaptation {
            self.apply_latency_scoring(network, &mut score_classical, &mut score_hybrid, &mut score_quantum, &mut reasoning);
        }

        // CPU considerations
        if self.config.enable_cpu_adaptation {
            self.apply_cpu_scoring(device, &mut score_classical, &mut score_hybrid, &mut score_quantum, &mut reasoning);
        }

        // Device class considerations
        self.apply_device_class_scoring(device, &mut score_classical, &mut score_hybrid, &mut score_quantum, &mut reasoning);

        // Determine the best mode
        let (recommended_mode, confidence) = if score_classical >= score_hybrid && score_classical >= score_quantum {
            (CryptoMode::Classical, score_classical)
        } else if score_hybrid >= score_quantum {
            (CryptoMode::Hybrid, score_hybrid)
        } else {
            (CryptoMode::Quantum, score_quantum)
        };

        let (performance_impact, battery_impact, bandwidth_impact) = 
            self.calculate_impact_estimates(recommended_mode);

        AdaptiveRecommendation {
            recommended_mode,
            confidence,
            reasoning,
            expected_performance_impact: performance_impact,
            expected_battery_impact: battery_impact,
            expected_bandwidth_impact: bandwidth_impact,
            valid_for_seconds: 60, // Valid for 1 minute
        }
    }

    fn apply_bandwidth_scoring(
        &self,
        network: &NetworkConditions,
        score_classical: &mut f64,
        score_hybrid: &mut f64,
        score_quantum: &mut f64,
        reasoning: &mut Vec<String>,
    ) {
        if network.bandwidth_mbps < 1.0 {
            // Very low bandwidth - prefer classical
            *score_classical += 0.3;
            *score_hybrid -= 0.2;
            *score_quantum -= 0.1;
            reasoning.push("Low bandwidth favors classical crypto".to_string());
        } else if network.bandwidth_mbps < 5.0 {
            // Moderate bandwidth - slight preference for quantum over hybrid
            *score_quantum += 0.1;
            *score_hybrid -= 0.1;
            reasoning.push("Moderate bandwidth favors quantum over hybrid".to_string());
        } else {
            // High bandwidth - security can take precedence
            *score_hybrid += 0.2;
            reasoning.push("High bandwidth allows for maximum security".to_string());
        }

        if network.is_metered {
            // Metered connection - minimize data usage
            *score_classical += 0.2;
            *score_hybrid -= 0.3;
            *score_quantum -= 0.1;
            reasoning.push("Metered connection reduces crypto overhead preference".to_string());
        }
    }

    fn apply_battery_scoring(
        &self,
        device: &DeviceConstraints,
        score_classical: &mut f64,
        score_hybrid: &mut f64,
        score_quantum: &mut f64,
        reasoning: &mut Vec<String>,
    ) {
        match device.power_source {
            PowerSource::Battery => {
                if device.battery_level_percent < 20.0 {
                    // Critical battery - prioritize efficiency
                    *score_classical += 0.4;
                    *score_hybrid -= 0.3;
                    *score_quantum -= 0.1;
                    reasoning.push("Critical battery level favors classical crypto".to_string());
                } else if device.battery_level_percent < 50.0 {
                    // Low battery - moderate efficiency preference
                    *score_classical += 0.2;
                    *score_hybrid -= 0.2;
                    reasoning.push("Low battery level favors efficient crypto".to_string());
                }
            }
            PowerSource::PluggedIn => {
                // Plugged in - can prioritize security
                *score_hybrid += 0.2;
                reasoning.push("Plugged in allows maximum security".to_string());
            }
            PowerSource::Charging => {
                // Charging - moderate security preference
                *score_quantum += 0.1;
                reasoning.push("Charging allows good security".to_string());
            }
        }

        match device.thermal_state {
            ThermalState::Hot | ThermalState::Critical => {
                // High temperature - reduce CPU intensive operations
                *score_classical += 0.3;
                *score_hybrid -= 0.4;
                *score_quantum -= 0.2;
                reasoning.push("High thermal state favors low-CPU crypto".to_string());
            }
            ThermalState::Warm => {
                *score_classical += 0.1;
                *score_hybrid -= 0.1;
                reasoning.push("Elevated temperature slightly favors classical".to_string());
            }
            ThermalState::Normal => {
                // Normal temperature - no adjustment needed
            }
        }
    }

    fn apply_latency_scoring(
        &self,
        network: &NetworkConditions,
        score_classical: &mut f64,
        score_hybrid: &mut f64,
        score_quantum: &mut f64,
        reasoning: &mut Vec<String>,
    ) {
        if network.latency_ms > 500.0 {
            // High latency - minimize round trips and processing time
            *score_classical += 0.2;
            *score_hybrid -= 0.2;
            *score_quantum += 0.1;
            reasoning.push("High latency favors efficient crypto operations".to_string());
        }

        match network.connection_stability {
            ConnectionStability::VeryUnstable => {
                // Unstable connection - prioritize reliability and speed
                *score_classical += 0.3;
                *score_hybrid -= 0.2;
                *score_quantum -= 0.1;
                reasoning.push("Unstable connection favors reliable classical crypto".to_string());
            }
            ConnectionStability::Unstable => {
                *score_classical += 0.1;
                *score_hybrid -= 0.1;
                reasoning.push("Somewhat unstable connection favors classical".to_string());
            }
            ConnectionStability::Stable => {
                // Stable connection - can use more complex crypto
                *score_hybrid += 0.1;
                reasoning.push("Stable connection allows complex crypto".to_string());
            }
        }
    }

    fn apply_cpu_scoring(
        &self,
        device: &DeviceConstraints,
        score_classical: &mut f64,
        score_hybrid: &mut f64,
        score_quantum: &mut f64,
        reasoning: &mut Vec<String>,
    ) {
        if device.cpu_usage_percent > 80.0 {
            // High CPU usage - prefer less intensive crypto
            *score_classical += 0.3;
            *score_hybrid -= 0.4;
            *score_quantum -= 0.1;
            reasoning.push("High CPU usage favors lightweight crypto".to_string());
        } else if device.cpu_usage_percent > 60.0 {
            *score_classical += 0.1;
            *score_hybrid -= 0.2;
            reasoning.push("Elevated CPU usage favors efficient crypto".to_string());
        } else if device.cpu_usage_percent < 30.0 {
            // Low CPU usage - can afford more intensive crypto
            *score_hybrid += 0.2;
            reasoning.push("Low CPU usage allows intensive crypto".to_string());
        }

        if device.memory_usage_percent > 85.0 {
            // High memory pressure - prefer crypto with lower memory overhead
            *score_classical += 0.2;
            *score_hybrid -= 0.3;
            *score_quantum -= 0.1;
            reasoning.push("High memory usage favors memory-efficient crypto".to_string());
        }
    }

    fn apply_device_class_scoring(
        &self,
        device: &DeviceConstraints,
        score_classical: &mut f64,
        score_hybrid: &mut f64,
        score_quantum: &mut f64,
        reasoning: &mut Vec<String>,
    ) {
        match device.device_class {
            DeviceClass::Mobile => {
                // Mobile devices - balance security and efficiency
                *score_quantum += 0.1;
                reasoning.push("Mobile device favors balanced quantum crypto".to_string());
            }
            DeviceClass::IoT => {
                // IoT devices - prioritize efficiency
                *score_classical += 0.3;
                *score_hybrid -= 0.2;
                *score_quantum -= 0.1;
                reasoning.push("IoT device favors efficient classical crypto".to_string());
            }
            DeviceClass::Server => {
                // Servers - can afford maximum security
                *score_hybrid += 0.3;
                reasoning.push("Server can afford maximum security".to_string());
            }
            DeviceClass::Desktop => {
                // Desktop - good balance, slight security preference
                *score_hybrid += 0.1;
                reasoning.push("Desktop allows good security".to_string());
            }
            DeviceClass::Laptop => {
                // Laptop - depends on power source (handled in battery scoring)
                reasoning.push("Laptop crypto choice depends on power source".to_string());
            }
            DeviceClass::Tablet => {
                // Tablet - similar to mobile but potentially more powerful
                *score_quantum += 0.05;
                reasoning.push("Tablet favors quantum crypto".to_string());
            }
        }
    }

    fn calculate_impact_estimates(&self, mode: CryptoMode) -> (f64, f64, f64) {
        match mode {
            CryptoMode::Classical => (1.0, 1.0, 1.0), // Baseline
            CryptoMode::Hybrid => (1.8, 1.6, 1.2),    // Higher impact
            CryptoMode::Quantum | CryptoMode::QuantumSafe => (1.4, 1.3, 1.1),   // Moderate impact
        }
    }

    fn fallback_recommendation(&self) -> AdaptiveRecommendation {
        AdaptiveRecommendation {
            recommended_mode: self.config.fallback_mode,
            confidence: 0.5,
            reasoning: vec!["Fallback recommendation due to insufficient data".to_string()],
            expected_performance_impact: 1.0,
            expected_battery_impact: 1.0,
            expected_bandwidth_impact: 1.0,
            valid_for_seconds: 30,
        }
    }

    fn cleanup_old_measurements(&mut self) {
        let window = Duration::from_secs(self.config.measurement_window_seconds);
        let cutoff = Instant::now() - window;

        while let Some(measurement) = self.measurements.front() {
            if measurement.timestamp < cutoff {
                self.measurements.pop_front();
            } else {
                break;
            }
        }
    }

    fn analyze_trends(&self, measurements: &[&Measurement]) -> PerformanceTrends {
        if measurements.len() < self.config.min_samples_for_decision {
            return PerformanceTrends::default();
        }

        let mut trends_by_mode: std::collections::HashMap<CryptoMode, Vec<&Measurement>> = std::collections::HashMap::new();

        // Group measurements by crypto mode
        for measurement in measurements {
            let mode_measurements = trends_by_mode
                .entry(measurement.crypto_mode)
                .or_insert_with(Vec::new);
            mode_measurements.push(measurement);
        }

        // Calculate trends for each mode
        let mut mode_trends = std::collections::HashMap::new();
        for (mode, mode_measurements) in trends_by_mode {
            if mode_measurements.len() >= 2 {
                let trend = self.calculate_mode_trend(&mode_measurements);
                mode_trends.insert(mode, trend);
            }
        }

        PerformanceTrends {
            mode_trends,
            overall_trend: TrendDirection::Stable, // Simplified
            confidence: 0.7,
        }
    }

    fn calculate_mode_trend(&self, measurements: &[&Measurement]) -> ModeTrend {
        let n = measurements.len();
        if n < 2 {
            return ModeTrend::default();
        }

        // Calculate average performance metrics with explicit type handling
        let total_encryption_time: f64 = measurements
            .iter()
            .map(|m| m.performance_metrics.encryption_time_ms)
            .fold(0.0, |acc, x| acc + x);
        let avg_encryption_time = total_encryption_time / (n as f64);

        let total_cpu_usage: f64 = measurements
            .iter()
            .map(|m| m.performance_metrics.cpu_usage_percent)
            .fold(0.0, |acc, x| acc + x);
        let avg_cpu_usage = total_cpu_usage / (n as f64);

        let total_memory_usage: f64 = measurements
            .iter()
            .map(|m| m.performance_metrics.memory_usage_mb)
            .fold(0.0, |acc, x| acc + x);
        let avg_memory_usage = total_memory_usage / (n as f64);

        let total_battery_drain: f64 = measurements
            .iter()
            .map(|m| m.performance_metrics.battery_drain_mw)
            .fold(0.0, |acc, x| acc + x);
        let avg_battery_drain = total_battery_drain / (n as f64);

        // Simple trend detection (first vs last half comparison)
        let split_point = n / 2;
        let first_half_total: f64 = measurements[..split_point]
            .iter()
            .map(|m| m.performance_metrics.encryption_time_ms)
            .fold(0.0, |acc, x| acc + x);
        let first_half_perf = first_half_total / (split_point as f64);
        
        let second_half_total: f64 = measurements[split_point..]
            .iter()
            .map(|m| m.performance_metrics.encryption_time_ms)
            .fold(0.0, |acc, x| acc + x);
        let second_half_perf = second_half_total / ((n - split_point) as f64);

        let performance_change = (second_half_perf - first_half_perf) / first_half_perf;
        let trend_direction = if performance_change > self.config.adaptation_threshold {
            TrendDirection::Degrading
        } else if performance_change < -self.config.adaptation_threshold {
            TrendDirection::Improving
        } else {
            TrendDirection::Stable
        };

        ModeTrend {
            avg_encryption_time_ms: avg_encryption_time,
            avg_cpu_usage_percent: avg_cpu_usage,
            avg_memory_usage_mb: avg_memory_usage,
            avg_battery_drain_mw: avg_battery_drain,
            trend_direction,
            change_rate: performance_change,
            sample_count: n,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceTrends {
    pub mode_trends: std::collections::HashMap<CryptoMode, ModeTrend>,
    pub overall_trend: TrendDirection,
    pub confidence: f64,
}

#[derive(Debug, Clone, Default)]
pub struct ModeTrend {
    pub avg_encryption_time_ms: f64,
    pub avg_cpu_usage_percent: f64,
    pub avg_memory_usage_mb: f64,
    pub avg_battery_drain_mw: f64,
    pub trend_direction: TrendDirection,
    pub change_rate: f64,
    pub sample_count: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TrendDirection {
    Improving,
    #[default]
    Stable,
    Degrading,
}

/// Utility functions for measuring network conditions
pub struct NetworkMeasurement;

impl NetworkMeasurement {
    /// Estimate current network conditions (simplified for Session 6)
    pub fn measure_current_conditions() -> NetworkConditions {
        // In a real implementation, this would:
        // - Measure actual bandwidth using test transfers
        // - Ping target servers to measure latency
        // - Check connection type and signal strength
        // - Detect if connection is metered
        
        NetworkConditions {
            bandwidth_mbps: 10.0, // Default assumption
            latency_ms: 50.0,
            packet_loss_rate: 0.01,
            connection_stability: ConnectionStability::Stable,
            is_metered: false,
            signal_strength: SignalStrength::Good,
        }
    }
    
    /// Perform a quick bandwidth test
    pub async fn quick_bandwidth_test() -> f64 {
        // Simplified bandwidth test - in reality would transfer test data
        // and measure throughput
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        10.0 // Return default value
    }
    
    /// Measure round-trip latency
    pub async fn measure_latency(_target: &str) -> f64 {
        let start = std::time::Instant::now();
        // In reality, would ping the target
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        start.elapsed().as_millis() as f64
    }
}

/// Utility functions for measuring device constraints
pub struct DeviceMeasurement;

impl DeviceMeasurement {
    /// Get current device constraints
    pub fn measure_current_constraints() -> DeviceConstraints {
        // In a real implementation, this would:
        // - Read battery level from system APIs
        // - Monitor CPU and memory usage
        // - Check thermal state
        // - Detect power source
        // - Identify device class
        
        DeviceConstraints {
            battery_level_percent: 75.0,
            cpu_usage_percent: 30.0,
            memory_usage_percent: 45.0,
            thermal_state: ThermalState::Normal,
            power_source: PowerSource::Battery,
            device_class: DeviceClass::Laptop,
        }
    }
    
    /// Monitor performance metrics during crypto operation
    pub fn measure_crypto_performance<F, R>(operation: F) -> (R, PerformanceMetrics)
    where
        F: FnOnce() -> R,
    {
        let start_time = std::time::Instant::now();
        let start_cpu = Self::get_cpu_usage();
        let start_memory = Self::get_memory_usage();
        
        let result = operation();
        
        let elapsed = start_time.elapsed();
        let end_cpu = Self::get_cpu_usage();
        let end_memory = Self::get_memory_usage();
        
        let metrics = PerformanceMetrics {
            encryption_time_ms: elapsed.as_millis() as f64,
            decryption_time_ms: 0.0, // Would be measured separately
            message_size_bytes: 1024, // Would be actual message size
            cpu_usage_percent: end_cpu - start_cpu,
            memory_usage_mb: end_memory - start_memory,
            battery_drain_mw: 10.0, // Would be measured from battery APIs
        };
        
        (result, metrics)
    }
    
    fn get_cpu_usage() -> f64 {
        // Simplified CPU usage measurement
        30.0
    }
    
    fn get_memory_usage() -> f64 {
        // Simplified memory usage measurement  
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_config_default() {
        let config = AdaptiveConfig::default();
        assert!(config.enable_bandwidth_adaptation);
        assert!(config.enable_battery_adaptation);
        assert_eq!(config.measurement_window_seconds, 300);
        assert_eq!(config.fallback_mode, CryptoMode::Classical);
    }

    #[test]
    fn test_network_conditions() {
        let conditions = NetworkConditions {
            bandwidth_mbps: 5.0,
            latency_ms: 100.0,
            packet_loss_rate: 0.02,
            connection_stability: ConnectionStability::Stable,
            is_metered: true,
            signal_strength: SignalStrength::Good,
        };
        
        assert_eq!(conditions.bandwidth_mbps, 5.0);
        assert!(conditions.is_metered);
    }

    #[test]
    fn test_device_constraints() {
        let device = DeviceConstraints {
            battery_level_percent: 45.0,
            cpu_usage_percent: 75.0,
            memory_usage_percent: 60.0,
            thermal_state: ThermalState::Warm,
            power_source: PowerSource::Battery,
            device_class: DeviceClass::Mobile,
        };
        
        assert_eq!(device.battery_level_percent, 45.0);
        assert!(matches!(device.device_class, DeviceClass::Mobile));
    }

    #[test]
    fn test_adaptive_mode_selector() {
        let config = AdaptiveConfig::default();
        let mut selector = AdaptiveModeSelector::new(config);
        
        let network = NetworkConditions {
            bandwidth_mbps: 1.0, // Low bandwidth
            latency_ms: 200.0,
            packet_loss_rate: 0.05,
            connection_stability: ConnectionStability::Unstable,
            is_metered: true,
            signal_strength: SignalStrength::Poor,
        };
        
        let device = DeviceConstraints {
            battery_level_percent: 15.0, // Critical battery
            cpu_usage_percent: 85.0,     // High CPU
            memory_usage_percent: 90.0,  // High memory
            thermal_state: ThermalState::Hot,
            power_source: PowerSource::Battery,
            device_class: DeviceClass::Mobile,
        };
        
        let recommendation = selector.recommend_mode(&network, &device);
        
        // With these poor conditions, should recommend classical
        assert_eq!(recommendation.recommended_mode, CryptoMode::Classical);
        assert!(!recommendation.reasoning.is_empty());
        assert!(recommendation.confidence > 0.0);
    }

    #[test]
    fn test_high_performance_conditions() {
        let config = AdaptiveConfig::default();
        let mut selector = AdaptiveModeSelector::new(config);
        
        let network = NetworkConditions {
            bandwidth_mbps: 100.0, // High bandwidth
            latency_ms: 10.0,
            packet_loss_rate: 0.001,
            connection_stability: ConnectionStability::Stable,
            is_metered: false,
            signal_strength: SignalStrength::Excellent,
        };
        
        let device = DeviceConstraints {
            battery_level_percent: 95.0,
            cpu_usage_percent: 20.0,     // Low CPU
            memory_usage_percent: 30.0,  // Low memory
            thermal_state: ThermalState::Normal,
            power_source: PowerSource::PluggedIn,
            device_class: DeviceClass::Desktop,
        };
        
        let recommendation = selector.recommend_mode(&network, &device);
        
        // With these good conditions, should recommend hybrid for max security
        assert_eq!(recommendation.recommended_mode, CryptoMode::Hybrid);
        assert!(recommendation.confidence > 0.7);
    }

    #[test]
    fn test_performance_metrics_recording() {
        let config = AdaptiveConfig::default();
        let mut selector = AdaptiveModeSelector::new(config);
        
        let network = NetworkMeasurement::measure_current_conditions();
        let device = DeviceMeasurement::measure_current_constraints();
        
        let metrics = PerformanceMetrics {
            encryption_time_ms: 5.0,
            decryption_time_ms: 3.0,
            message_size_bytes: 1024,
            cpu_usage_percent: 15.0,
            memory_usage_mb: 50.0,
            battery_drain_mw: 20.0,
        };
        
        selector.record_performance(network, device, CryptoMode::Classical, metrics);
        
        assert_eq!(selector.measurements.len(), 1);
        
        let trends = selector.get_performance_trends();
        // Should have insufficient data for trends initially
        assert!(trends.mode_trends.is_empty());
    }

    #[test]
    fn test_measurement_cleanup() {
        let mut config = AdaptiveConfig::default();
        config.measurement_window_seconds = 1; // Very short window for testing
        
        let mut selector = AdaptiveModeSelector::new(config);
        
        let network = NetworkMeasurement::measure_current_conditions();
        let device = DeviceMeasurement::measure_current_constraints();
        let metrics = PerformanceMetrics {
            encryption_time_ms: 5.0,
            decryption_time_ms: 3.0,
            message_size_bytes: 1024,
            cpu_usage_percent: 15.0,
            memory_usage_mb: 50.0,
            battery_drain_mw: 20.0,
        };
        
        // Add measurement
        selector.record_performance(network.clone(), device.clone(), CryptoMode::Classical, metrics.clone());
        assert_eq!(selector.measurements.len(), 1);
        
        // Wait for expiration and add another measurement
        std::thread::sleep(std::time::Duration::from_millis(1100));
        selector.record_performance(network, device, CryptoMode::Classical, metrics);
        
        // Should have cleaned up old measurement
        assert_eq!(selector.measurements.len(), 1);
    }
}
