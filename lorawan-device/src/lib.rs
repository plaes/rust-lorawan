#![cfg_attr(not(test), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(async_fn_in_trait)]

//! ## Feature flags
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]
#![doc = include_str!("../README.md")]

use core::default::Default;
use heapless::Vec;

mod radio;

pub mod mac;
use mac::NetworkCredentials;

pub mod region;
pub use region::Region;

#[cfg(test)]
mod test_util;

pub mod nb_device;
use nb_device::state::State;

use core::marker::PhantomData;
#[cfg(feature = "default-crypto")]
pub use lorawan::default_crypto;
pub use lorawan::{
    keys::{AppEui, AppKey, AppSKey, CryptoFactory, DevEui, NewSKey},
    parser::DevAddr,
};

pub use rand_core::RngCore;
mod rng;
pub use rng::Prng;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_device;

#[derive(Debug)]
/// Provides the application payload and FPort of a downlink message.
pub struct Downlink {
    pub data: Vec<u8, 256>,
    pub fport: u8,
}

#[cfg(feature = "defmt")]
impl defmt::Format for Downlink {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Downlink {{ fport: {}, data: ", self.fport,);

        for byte in self.data.iter() {
            defmt::write!(f, "{:02x}", byte);
        }
        defmt::write!(f, " }}")
    }
}

/// Allows to fine-tune the beginning and end of the receive windows for a specific board and runtime.
pub trait Timings {
    /// How many milliseconds before the RX window should the SPI transaction start?
    /// This value needs to account for the time it takes to wake up the radio and start the SPI transaction, as
    /// well as any non-deterministic delays in the system.
    fn get_rx_window_lead_time_ms(&self) -> u32;

    /// Explicitly set the amount of milliseconds to listen before the window starts. By default, the pessimistic assumption
    /// of `Self::get_rx_window_lead_time_ms` will be used. If you override, be sure that: `Self::get_rx_window_buffer
    /// < Self::get_rx_window_lead_time_ms`.
    fn get_rx_window_buffer(&self) -> u32 {
        self.get_rx_window_lead_time_ms()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Join the network using either OTAA or ABP.
pub enum JoinMode {
    OTAA { deveui: DevEui, appeui: AppEui, appkey: AppKey },
    ABP { newskey: NewSKey, appskey: AppSKey, devaddr: DevAddr<[u8; 4]> },
}
