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
            map.insert(String::from("amount"), amount.to_string());
        }

        if let Some(label) = self.label.clone() {
            map.insert(String::from("label"), label);
        }

        if let Some(message) = self.message.clone() {
            map.insert(String::from("message"), message);
        }

        if let Some(params) = self.params.clone() {
            for (key, value) in params {
                map.insert(key.clone(), value.clone());
            }
        }

        match Url::parse_with_params(&url, map) {
            Ok(parsed) => Ok(parsed.as_str().to_string()),
            Err(_) => Err(Error::UrlParseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use url::Url;
    use super::*;

    #[test]
    fn test_build_uri() {
        let mut params = HashMap::new();
        params.insert(String::from("somethingyoudontunderstan"), String::from("50"));
        params.insert(String::from("somethingelseyoudontget"), String::from("999"));
        let uri = URIResources::new(
            String::from("bitcoin"),
            String::from("175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W"),
            Some(100.0),
            Some(String::from("Luke-Jr")),
            Some(String::from("message")),
            Some(params),
        );
        let url = Url::parse(&uri.build_uri().unwrap()).unwrap();
        assert_eq!(url.scheme(), "bitcoin");
        assert_eq!(url.path(), "175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W");
        assert_eq!(url.query_pairs().count(), 5);
    }
}
