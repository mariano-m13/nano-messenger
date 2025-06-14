// Minimal compilation test for our fixes
use crate::media::{
    compatibility::mobile::{MobileOptimization, MobileQualityLevel},
    processing::MediaType,
};

// Test that our main types compile
pub fn test_mobile_types() {
    let _quality = MobileQualityLevel {
        name: "Test".to_string(),
        width: 1920,
        height: 1080,
        bitrate: 5000000,
        framerate: 30.0,
    };

    // Test MediaType variants
    let _video = MediaType::Video;
    let _audio = MediaType::Audio;
    let _image = MediaType::Image;
    let _document = MediaType::Document;
    let _unknown = MediaType::Unknown;
}
