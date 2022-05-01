#[cfg(target_os = "linux")]
mod linux;
#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use crate::windows::*;
#[cfg(target_os = "linux")]
pub use linux::*;
