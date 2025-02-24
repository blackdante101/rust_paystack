use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE},
    Client,
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Clone, Debug)]
pub struct RustPaystack {
    api_key: String,
    client: Client,
    base_url: Url,
}

#[derive(Serialize)]
pub struct InitializeTransactionRequest<'a> {
    email: &'a str,
    amount: Decimal,
}

#[derive(Deserialize, Debug)]
pub struct InitializeTransactionResponse {
    status: bool,
    message: String,
    data: TransactionData,
}

#[derive(Deserialize, Debug)]
pub struct TransactionData {
    authorization_url: String,
    access_code: String,
    reference: String,
}

#[derive(Deserialize, Debug)]
pub struct VerifyTransactionResponse {
    status: bool,
    message: String,
    data: Option<serde_json::Value>,
}

impl RustPaystack {
    pub fn new(api_key: String) -> RustPaystack {
        let client = reqwest::Client::new();
        let base_url = Url::parse("https://api.paystack.co").unwrap();

        return RustPaystack {
            api_key: api_key,
            client: client,
            base_url: base_url,
        };
    }

    pub async fn initialize_transaction(
        &self,
        email: &str,
        amount: Decimal,
    ) -> Result<InitializeTransactionResponse, RustPaystackError> {
        /*
           Paystack makes use of the ISO 4217 format for currency codes. When sending an amount, it must be sent in the subunit of that currency.
           Sending an amount in subunits simply means multiplying the base amount by 100.
           For example, if a customer is supposed to make a payment of NGN 100, you would send 10000 = 100 * 100 in your request.
        */

        let final_amount = amount * dec!(100.0);
        let req_url = self.base_url.join("/transaction/initialize")?;

        //creates a json object
        let postdata = InitializeTransactionRequest {
            email: &email,
            amount: final_amount,
        };

        let response = self
            .client
            .post(req_url)
            .header(AUTHORIZATION, format!("Bearer {}", &self.api_key))
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(CACHE_CONTROL, "no-cache")
            .json(&postdata)
            .send()
            .await?
            .json::<InitializeTransactionResponse>()
            .await?;

        Ok(response)
    }

    pub async fn verify_payment(
        &self,
        reference: &str,
    ) -> Result<VerifyTransactionResponse, RustPaystackError> {
        let req_url = self
            .base_url
            .join(&format!("/transaction/verify/{}", reference))?;

        let response = self
            .client
            .get(req_url.to_string())
            .header(AUTHORIZATION, format!("Bearer {}", &self.api_key))
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(CACHE_CONTROL, "no-cache")
            .send()
            .await?
            .json::<VerifyTransactionResponse>()
            .await?;

        Ok(response)
    }
}

#[derive(Debug, Error)]
pub enum RustPaystackError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error), // Handles network errors

    #[error("Invalid URL error: {0}")]
    UrlParseError(#[from] url::ParseError), // Handles incorrect URLs

    #[error("Paystack API error: {0}")]
    PaystackError(String), // Custom API response errors
}
