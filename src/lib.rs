#![no_std]

extern crate alloc;

pub mod ffi {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(clippy::all)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// Optional higher-level wrapper goes behind a feature once stabilized for your VkFFT version.
#[cfg(feature = "wrapper")]
pub mod vkfft;
#[cfg(feature = "wrapper")]
pub use vkfft::VkFft;
