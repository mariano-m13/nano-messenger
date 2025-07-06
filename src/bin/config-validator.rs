//! Configuration Validator Binary for Quantum-Resistant Nano-Messenger
//! 
//! This binary validates production configuration files to ensure they meet
//! security, compliance, and operational requirements before deployment.


use clap::{Arg, Command};
use nano_messenger::production::config_validation::{
    load_and_validate_config, ConfigValidator, Environment, ValidationResult
};
use std::path::PathBuf;
use std::process;

fn main() {
    let matches = Command::new("config-validator")
        .version("2.0.0")
        .author("Quantum-Resistant Nano-Messenger Team")
        .about("Validates production configuration files for deployment")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Path to configuration file")
                .required(true)
        )
        .arg(
            Arg::new("environment")
                .short('e')
                .long("environment")
                .value_name("ENV")
                .help("Target environment (development, testing, staging, production)")
                .required(true)
        )
        .arg(
            Arg::new("strict")
                .short('s')
                .long("strict")
                .help("Enable strict validation mode")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("output-format")
                .short('o')
                .long("output-format")
                .value_name("FORMAT")
                .help("Output format (text, json)")
                .default_value("text")
        )
        .arg(
            Arg::new("security-baseline")
                .long("security-baseline")
                .help("Show security baseline for the environment")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let config_path = PathBuf::from(matches.get_one::<String>("config").unwrap());
    let environment_str = matches.get_one::<String>("environment").unwrap();
    let strict_mode = matches.get_flag("strict");
    let output_format = matches.get_one::<String>("output-format").unwrap();
    let show_baseline = matches.get_flag("security-baseline");

    // Parse environment
    let environment = match environment_str.to_lowercase().as_str() {
        "development" | "dev" => Environment::Development,
        "testing" | "test" => Environment::Testing,
        "staging" | "stage" => Environment::Staging,
        "production" | "prod" => Environment::Production,
        _ => {
            eprintln!("Error: Invalid environment '{}'. Valid options: development, testing, staging, production", environment_str);
            process::exit(1);
        }
    };

    // Check if config file exists
    if !config_path.exists() {
        eprintln!("Error: Configuration file '{}' not found", config_path.display());
        process::exit(1);
    }

    // Show security baseline if requested
    if show_baseline {
        show_security_baseline(&environment, output_format);
        return;
    }

    // Load and validate configuration
    match load_and_validate_config(&config_path, environment.clone()) {
        Ok(config) => {
            println!("✅ Configuration validation passed!");
            
            // Additional validation with custom validator for more detailed output
            let validator = ConfigValidator::new(environment.clone());
            let validation_result = validator.validate(&config);
            
            output_validation_result(&validation_result, output_format, strict_mode);
            
            if !validation_result.valid || (strict_mode && !validation_result.warnings.is_empty()) {
                process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("❌ Configuration validation failed:");
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}

fn show_security_baseline(environment: &Environment, output_format: &str) {
    use nano_messenger::production::config_validation::SecurityBaseline;
    
    let baseline = SecurityBaseline::for_environment(environment);
    
    match output_format {
        "json" => {
            // Simple JSON output since we can't derive Serialize easily
            println!("{{");
            println!("  \"environment\": \"{:?}\",", environment);
            println!("  \"min_tls_version\": \"{}\",", baseline.min_tls_version);
            println!("  \"min_key_size\": {},", baseline.min_key_size);
            println!("  \"require_perfect_forward_secrecy\": {},", baseline.require_perfect_forward_secrecy);
            println!("  \"max_session_timeout_minutes\": {},", baseline.max_session_timeout_minutes);
            println!("  \"require_audit_logging\": {}", baseline.require_audit_logging);
            println!("}}");
        }
        _ => {
            println!("Security Baseline for {:?} Environment:", environment);
            println!("=====================================");
            println!("Minimum TLS Version: {}", baseline.min_tls_version);
            println!("Minimum Key Size: {} bits", baseline.min_key_size);
            println!("Perfect Forward Secrecy: {}", if baseline.require_perfect_forward_secrecy { "Required" } else { "Optional" });
            println!("Max Session Timeout: {} minutes", baseline.max_session_timeout_minutes);
            println!("Audit Logging: {}", if baseline.require_audit_logging { "Required" } else { "Optional" });
            
            if !baseline.required_cipher_suites.is_empty() {
                println!("Required Cipher Suites:");
                for suite in &baseline.required_cipher_suites {
                    println!("  - {}", suite);
                }
            }
        }
    }
}

fn output_validation_result(result: &ValidationResult, output_format: &str, strict_mode: bool) {
    match output_format {
        "json" => {
            output_json_result(result);
        }
        _ => {
            output_text_result(result, strict_mode);
        }
    }
}

fn output_json_result(result: &ValidationResult) {
    println!("{{");
    println!("  \"valid\": {},", result.valid);
    println!("  \"security_score\": {},", result.security_score);
    
    // Errors
    println!("  \"errors\": [");
    for (i, error) in result.errors.iter().enumerate() {
        let comma = if i < result.errors.len() - 1 { "," } else { "" };
        println!("    \"{}\"{}",  error.to_string().replace("\"", "\\""), comma);
    }
    println!("  ],");
    
    // Warnings
    println!("  \"warnings\": [");
    for (i, warning) in result.warnings.iter().enumerate() {
        let comma = if i < result.warnings.len() - 1 { "," } else { "" };
        println!("    \"{}\"{}",  warning.replace("\"", "\\""), comma);
    }
    println!("  ],");
    
    // Recommendations
    println!("  \"recommendations\": [");
    for (i, recommendation) in result.recommendations.iter().enumerate() {
        let comma = if i < result.recommendations.len() - 1 { "," } else { "" };
        println!("    \"{}\"{}",  recommendation.replace("\"", "\\""), comma);
    }
    println!("  ]");
    
    println!("}}");
}

fn output_text_result(result: &ValidationResult, strict_mode: bool) {
    println!();
    println!("🔍 Configuration Validation Report");
    println!("================================");
    println!("Overall Status: {}", if result.valid { "✅ VALID" } else { "❌ INVALID" });
    println!("Security Score: {}/100", result.security_score);
    
    // Security score interpretation
    match result.security_score {
        90..=100 => println!("Security Level: 🟢 Excellent"),
        80..=89 => println!("Security Level: 🟡 Good"),
        70..=79 => println!("Security Level: 🟠 Moderate"),
        60..=69 => println!("Security Level: 🔴 Poor"),
        _ => println!("Security Level: ⚠️ Critical Issues"),
    }
    
    println!();
    
    // Errors
    if !result.errors.is_empty() {
        println!("❌ ERRORS ({}):", result.errors.len());
        println!("================");
        for (i, error) in result.errors.iter().enumerate() {
            println!("{}. {}", i + 1, error);
        }
        println!();
    }
    
    // Warnings
    if !result.warnings.is_empty() {
        println!("⚠️  WARNINGS ({}):", result.warnings.len());
        println!("==================");
        for (i, warning) in result.warnings.iter().enumerate() {
            println!("{}. {}", i + 1, warning);
        }
        println!();
        
        if strict_mode {
            println!("🔴 Strict mode: Warnings treated as errors");
        }
    }
    
    // Recommendations
    if !result.recommendations.is_empty() {
        println!("💡 RECOMMENDATIONS ({}):", result.recommendations.len());
        println!("==========================");
        for (i, recommendation) in result.recommendations.iter().enumerate() {
            println!("{}. {}", i + 1, recommendation);
        }
        println!();
    }
    
    // Summary
    if result.valid && result.warnings.is_empty() {
        println!("🎉 Configuration is production-ready!");
    } else if result.valid {
        println!("✅ Configuration is valid but has {} warning(s)", result.warnings.len());
        if strict_mode {
            println!("🔴 Deployment blocked due to strict mode");
        }
    } else {
        println!("🚫 Configuration has {} error(s) and must be fixed before deployment", result.errors.len());
    }
}
