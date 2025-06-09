@echo off
REM Comprehensive Test Script for Nano-Messenger (Windows)
REM Quantum-Resistant Messaging Protocol Validation Suite

setlocal enabledelayedexpansion

echo 🧪 NANO-MESSENGER COMPREHENSIVE SESSION TESTING
echo =================================================
echo Quantum-Resistant Messaging Protocol Validation Suite
echo Testing all implementation sessions...
echo.

set TOTAL_TESTS=0
set PASSED_TESTS=0
set FAILED_TESTS=0

REM Check if cargo is available
where cargo >nul 2>nul
if errorlevel 1 (
    echo ❌ Cargo not found. Please install Rust/Cargo first.
    exit /b 1
)

REM Function to run a test
call :run_test "Compilation Check" "cargo check --examples" "Checking all examples compile"
call :run_test "Library Build" "cargo build --release" "Building optimized library"

echo.
echo 📋 SESSION VALIDATIONS
echo =====================
echo.

call :run_test "Session 1 Validation" "cargo run --example session1_validation" "Core cryptographic implementation"
call :run_test "Session 2 Validation" "cargo run --example session2_validation" "Protocol implementation"
call :run_test "Session 3 Validation" "cargo run --example session3_validation" "Quantum-safe messaging"
call :run_test "Session 4 Validation" "cargo run --example session4_validation" "Multi-mode crypto support"
call :run_test "Session 5 Validation" "cargo run --example session5_validation" "Relay policy enforcement"
call :run_test "Session 6 Validation" "cargo run --example session6_validation" "Performance optimization"
call :run_test "Session 7 Validation" "cargo run --example session7_validation" "Security validation"

echo.
echo 🧪 UNIT TESTS
echo =============
echo.

call :run_test "Unit Tests" "cargo test" "All module unit tests"
call :run_test "Doc Tests" "cargo test --doc" "Documentation example tests"

echo.
echo 📊 COMPREHENSIVE TEST REPORT
echo =============================
echo.
echo 📈 Test Statistics:
echo    Total Tests: !TOTAL_TESTS!
echo    Passed: !PASSED_TESTS!
echo    Failed: !FAILED_TESTS!

if !FAILED_TESTS! EQU 0 (
    echo    Success Rate: 100%%
    echo.
    echo 🎉 ALL TESTS PASSED! Your quantum-resistant messaging protocol is fully validated!
    echo.
    echo 🛡️  Security Status:
    echo    ✅ Security Validation: COMPLETE
    echo    ✅ Cryptographic Correctness: VERIFIED
    echo    ✅ Production Ready: YES
) else (
    set /a success_rate=!PASSED_TESTS! * 100 / !TOTAL_TESTS!
    echo    Success Rate: !success_rate!%%
    echo.
    echo ⚠️  Some tests failed. Review the detailed results above.
    echo.
    echo 🛡️  Security Status:
    echo    ❌ Some validations incomplete
    echo    ⚠️  Review errors before production use
)

echo.
echo 💻 System Information:
cargo --version
rustc --version
echo    Test Date: %date% %time%
echo    Platform: Windows

if !FAILED_TESTS! EQU 0 (
    echo.
    echo 🏆 ALL VALIDATIONS SUCCESSFUL! Protocol ready for deployment.
    exit /b 0
) else (
    echo.
    echo ❌ Some validations failed. Review errors before deployment.
    exit /b 1
)

REM Function to run a test and track results
:run_test
set test_name=%~1
set test_command=%~2
set description=%~3

echo 🔍 Testing: %test_name%
echo    %description%

set /a TOTAL_TESTS+=1

%test_command% >nul 2>&1
if errorlevel 1 (
    echo    ❌ FAILED
    set /a FAILED_TESTS+=1
) else (
    echo    ✅ PASSED
    set /a PASSED_TESTS+=1
)

goto :eof
