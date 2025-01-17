#![warn(bare_trait_objects, missing_docs, unreachable_pub)]
#![deny(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]

//! Rust Client/Wrapper built for [Alphavantage][alpha_vantage_link] API.
//!
//! [alpha_vantage_link]: https://alphavantage.co

pub mod crypto;

pub mod exchange;

pub mod forex;

pub mod quote;

pub mod search;

pub mod sector;

pub mod stock_time;

pub mod technical_indicator;

/// Module for basic definition of user information like setting API and
/// requesting through that API
pub mod user;

/// Utility module declaring enum for basic function and parameters for
/// different API
pub mod util;

use self::user::APIKey;

/// Set API value which can be used for calling different module
#[must_use]
pub fn set_api(api: &str) -> APIKey {
    APIKey::set_api(api)
}

/// Set API value with timeout period
#[must_use]
pub fn set_with_timeout(api: &str, timeout: u64) -> APIKey {
    APIKey::set_with_timeout(api, timeout)
}
