//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, set up the TEST_FOO environment variable with the current timestamp.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let your_command = format!("TEST_FOO={}", timestamp);
    println!("cargo:{}", your_command);

    // In tests8, enable the "pass" feature.
    let your_command = "cargo:rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}
