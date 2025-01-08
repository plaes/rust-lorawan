//! Compile-time configuration.
//!
//! `lorawan-device` features some configuration settings that are set at
//! compile time.
//!
//! These can be set in multiple of ways:
//!
//! - Via Cargo features: like `<feature>-<value>`. Name must be in lowercase
//!   and use dashes instead of underscores. For example `radio-buffer-size-256`.
//!   Only a selection of values is available, consult `Cargo.toml` for the list.
//! - Via environment variables at build time: set variable named `LORAWAN_DEVICE_<value>`.
//!   For example `LORAWAN_DEVICE_RADIO_BUFFER_SIZE=256 cargo build --release`
//! - Via `[env]` section of `.cargo/config.toml`.
//!
//! Note that environment variables take precedence over Cargo features. Also,
//! if multiple different values are enabled for same setting, compilation fails.

mod raw {
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

/// Radio buffer size.
///
/// This is the size of the radio buffer. Generally, this should be set to 256
/// to support the largest possible LoRa frames.
///
/// Default: 256.
pub const RADIO_BUFFER_SIZE: usize = raw::RADIO_BUFFER_SIZE;
