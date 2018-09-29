use RESULT_CERT_NAME;

#[cfg(windows)]
pub mod win32;

#[cfg(unix)]
pub mod nix;
