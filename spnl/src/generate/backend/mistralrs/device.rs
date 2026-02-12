//! Device detection and selection for mistral.rs backend
//!
//! This module handles automatic device selection with the following priority:
//! 1. CUDA GPU (when local-cuda feature is enabled)
//! 2. Metal GPU (on macOS)
//! 3. CPU (fallback)

use mistralrs::Device;

/// Check if logging is enabled via SPNL_LOG environment variable
fn should_enable_logging() -> bool {
    std::env::var("SPNL_LOG")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Detect and return the best available device for inference
///
/// Priority order:
/// 1. CUDA GPU (if local-cuda feature is enabled and CUDA is available)
/// 2. Metal GPU (on macOS if Metal is available)
/// 3. CPU (fallback)
///
/// # Returns
/// The best available Device for the current system configuration
pub fn detect_device() -> Device {
    // Try CUDA first if the feature is enabled
    #[cfg(feature = "local-cuda")]
    {
        match Device::new_cuda(0) {
            Ok(cuda_device) => {
                if should_enable_logging() {
                    eprintln!("Using CUDA GPU acceleration");
                }
                return cuda_device;
            }
            Err(e) => {
                if should_enable_logging() {
                    eprintln!("CUDA not available ({}), trying next option", e);
                }
                // Fall through to try Metal/CPU
            }
        }
    }

    // Try Metal on macOS
    if cfg!(target_os = "macos") {
        match Device::new_metal(0) {
            Ok(metal_device) => {
                if should_enable_logging() {
                    eprintln!("Using Metal GPU acceleration");
                }
                return metal_device;
            }
            Err(e) => {
                if should_enable_logging() {
                    eprintln!("Metal not available ({}), falling back to CPU", e);
                }
                // Fall through to CPU
            }
        }
    }

    // Fallback to CPU
    if should_enable_logging() {
        eprintln!("Using CPU for inference");
    }
    Device::Cpu
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_device_returns_valid_device() {
        // This test just ensures the function runs without panicking
        // The actual device returned depends on the system configuration
        let _device = detect_device();
    }
}

// Made with Bob
