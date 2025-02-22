//! # Rust Paystack
//!
//! a rust library for interacting with Paystack API
//! 
//! # Getting Started
//! run this command in your project directory
//! ```no_run
//! cargo add rust_paystack
//! ```
//! Including the library in your project:
//! ```no_run
//! use rust_paystack::Paystack;
//! ```
//! # Creating a new instance
//! when creating a new instance, api key should be parsed to string
//! ```no_run
//! let rust_p = RustPaystack::new(PAYSTACK_SECRET_KEY.to_string());
//! ```
//! # Initializing a transaction
//! ```no_run
//! #[tokio::main]
//!async fn main() {
//! let rust_p = RustPaystack::new(PAYSTACK_SECRET_KEY.to_string());
//! let req = rust_p.initialize_transaction( "test@testmail.com", 10.50).await;
//! 
//! println!("{:?}", req);
//!}
//! ```
//! 
//! # Verfiying a transaction
//! 
//! ```no_run
//! #[tokio::main]
//!async fn main() {
//! let rust_p = RustPaystack::new(PAYSTACK_SECRET_KEY.to_string());
//! let req = rust_p.verify_payment("reference").await;
//! 
//! println!("{:?}", req);
//!}
//! ```

mod payment;

pub use payment::RustPaystack;


