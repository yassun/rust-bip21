use std::collections::HashMap;
use url::{Url};

#[derive(Debug)]
pub enum Error {
	InvalidUrnErr,
	NegativeAmountErr,
    UrlParseError
}

#[derive(Debug)]
pub struct URIResources {
    urn_scheme: String,
	address: String,
	amount: Option<f64>,
	label: Option<String>,
	message: Option<String>,
    params: Option<HashMap<String, String>>,
}

impl URIResources {
    pub fn new(
        urn_scheme: String,
        address: String,
        amount: Option<f64>,
        label: Option<String>,
        message: Option<String>,
        params: Option<HashMap<String, String>>,
    ) -> URIResources {
        URIResources{
            urn_scheme,
            address,
            amount,
            label,
            message,
            params,
        }
    }

    pub fn build_uri(&self) -> Result<String, Error> {
        if self.urn_scheme != "bitcoin" {
            return Err(Error::InvalidUrnErr);
        }

        let mut url = format!( "{}{}", self.urn_scheme, ":" );
        url = format!( "{}{}", url, self.address );

        let mut map = HashMap::new();

        if let Some(amount) = self.amount {
            if amount < 0.0 {
                return Err(Error::NegativeAmountErr);
            }
            map.insert("amount", amount.to_string());
        }

        if let Some(label) = self.label.clone() {
            map.insert("label", label);
        }

        if let Some(message) = self.message.clone() {
            map.insert("message", message);
        }

        if let Some(params) = self.params.clone() {
            for (key, value) in params {
                map.insert(&key, value);
            }
        }

        match Url::parse_with_params(&url, map) {
            Ok(parsed) => Ok(parsed.as_str().to_string()),
            Err(_) => Err(Error::UrlParseError)
        }
    }
}
