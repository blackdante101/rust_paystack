use reqwest::header::{AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE, ACCEPT};
use serde_json::{json, Value};

pub struct RustPaystack<'a>{
    api_key: &'a str,
}

impl RustPaystack<'_> {

    pub fn new(api_key:&str) -> RustPaystack {
        return RustPaystack { api_key: api_key }
    }

    pub async fn initialize_transaction(&self, email: &str, amount: f64) -> Result<Value, Box<dyn std::error::Error>>{

        /*
            Paystack makes use of the ISO 4217 format for currency codes. When sending an amount, it must be sent in the subunit of that currency.
            Sending an amount in subunits simply means multiplying the base amount by 100. 
            For example, if a customer is supposed to make a payment of NGN 100, you would send 10000 = 100 * 100 in your request.
         */
        let final_amount = amount * 100.0; 

        //creates a json object 
        let postdata = json!({
            "email": &email,
            "amount": final_amount
        });


        let client = reqwest::Client::new();

        let post_req = client.post("https://api.paystack.co/transaction/initialize")
        .header(AUTHORIZATION, format!("Bearer {}", &self.api_key))
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(CACHE_CONTROL, "no-cache")
        .json(&postdata) 
        .send()
        .await?;

        
        let res_json = post_req.json().await?;

        Ok(res_json)
    } 


    pub async fn verify_payment(&self, reference: &str) -> Result<Value, Box<dyn std::error::Error>>{
        let verification_url = "https://api.paystack.co/transaction/verify/".to_owned()+reference;
        let client = reqwest::Client::new();

        let get_req= client.get(&verification_url)
        .header(AUTHORIZATION, format!("Bearer {}", &self.api_key))
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(CACHE_CONTROL, "no-cache")
        .send()
        .await?;

        let res_json= get_req.json().await?;

        Ok(res_json)
    }
}