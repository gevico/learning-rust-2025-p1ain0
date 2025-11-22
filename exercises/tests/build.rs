//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // tests8: 启用编译 cfg，使测试提前返回
    // Cargo 要求使用 cargo:rustc-cfg=KEY
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
