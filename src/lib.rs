//! # Rust Paystack
//!
//! A Rust library for interacting with the Paystack API.
//!
//! ## Getting Started
//! Run this command in your project directory:
//! ```rust,no_run
//! cargo add rust_paystack
//! cargo add rust_decimal_macros // for parsing the amount
//! ```
//!
//! ## Including the Library in Your Project
//! ```rust,no_run
//! use rust_paystack::Paystack;
//! ```
//!
//! ## Creating a New Instance
//! When creating a new instance, the API key should be passed as a string:
//! ```rust,no_run
//! let rust_p = RustPaystack::new("sk_xxxxxxxxxx".to_string());
//! ```
//!
//! ## Initializing a Transaction
//! ```rust,no_run
//! use rust_decimal_macros::dec;
//!
//! #[tokio::main]
//! async fn main() {
//!     let rust_p = RustPaystack::new("sk_xxxxxxxxxx".to_string());
//!
//!     let amount = dec!(10.50); // Amount should be parsed using rust_decimal_macros
//!     let email = "test@testmail.com";
//!
//!     let response = rust_p.initialize_transaction(email, amount).await;
//!
//!     println!("{:?}", response);
//! }
//! ```
//!
//! ## Verifying a Transaction
//! ```rust,no_run
//! #[tokio::main]
//! async fn main() {
//!     let rust_p = RustPaystack::new("sk_xxxxxxxxxx".to_string());
//!     let response = rust_p.verify_payment("reference").await;
//!
//!     println!("{:?}", response);
//! }
//! ```

mod payment;

pub use payment::RustPaystack;
