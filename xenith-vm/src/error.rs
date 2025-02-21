use thiserror::Error;

/// Error type for CPUID operations
///
/// This error type is used to represent errors that can occur when interacting with the CPUID
/// instruction.
#[derive(Error, Debug)]
pub enum CpuidError {
    #[error("Failed to get CPUID vendor info")]
    VendorInfo,
    #[error("Failed to get CPUID processor brand string")]
    ProcessorBrandString,
    #[error("Failed to convert a value to array")]
    ConversionError(String),
}
