/// Session 11: Cross-Platform Compatibility
/// 
/// Provides cross-platform media optimization and compatibility features
/// for mobile devices, web browsers, and desktop applications.

pub mod mobile;
pub mod web;

// Re-export main types
pub use mobile::{
    MobileOptimization, DeviceProfile, NetworkProfile, ProcessingStrategy,
    OptimizedMediaSet, BatteryAwareConfig, MobileQualityLevel
};
pub use web::{
    WebMediaSupport, BrowserCapabilities, WebMediaPackage, PWAManifest,
    WebOptimizationConfig, StreamingManifest
};
