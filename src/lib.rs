use std::collections::HashMap;
use url::Url;

#[derive(Debug)]
pub enum Error {
    InvalidUrnErr,
    NegativeAmountErr,
    UrlParseError,
    InvalidAmountErr,
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
        URIResources {
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

        let mut url = format!("{}{}", self.urn_scheme, ":");
        url = format!("{}{}", url, self.address);

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
            Err(_) => Err(Error::UrlParseError),
        }
    }
}

pub fn parse(uri: String) -> Result<URIResources, Error> {
    let s: Vec<&str> = uri.split(':').collect();
    if s[0] != "bitcoin" || s.len() != 2 {
        return Err(Error::InvalidUrnErr);
    }
    let urn = s[0].to_string();
    let address = parse_address(&uri, &urn);
    let mut p = parse_params(&uri, &urn, &address);
    let mut u = URIResources::new(urn, address, None, None, None, None);
    if let Some(amount) = p.get("amount") {
        u.amount = Some(parse_amount(amount)?);
        p.remove("amount");
    }

    if let Some(label) = p.get("label") {
        u.label = Some(String::from(label));
        p.remove("label");
    }

    if let Some(message) = p.get("message") {
        u.message = Some(String::from(message));
        p.remove("message");
    }

    u.params = Some(p);

    Ok(u)
}

fn parse_address(uri: &str, urn: &str) -> String {
    match uri.find("?") {
        Some(idx) => uri[urn.len() + 1..idx].to_string(),
        None => uri[urn.len() + 1..].to_string(),
    }
}

fn parse_params(uri: &str, urn: &str, address: &str) -> HashMap<String, String> {
    let mut ps = HashMap::new();
    let idx = urn.len() + 1 + address.len() + 1;
    if uri.len() < idx {
        return ps;
    }
    let qp = uri[idx..].to_string();
    let query: Vec<&str> = qp.split('&').collect();
    for q in query {
        let p: Vec<&str> = q.split('=').collect();
        if p.len() != 2 {
            continue;
        }
        ps.insert(String::from(p[0]), String::from(p[1]));
    }
    ps
}

fn parse_amount(amount: &str) -> Result<f64, Error> {
    match amount.parse::<f64>() {
        Ok(f) => {
            if f < 0.0 {
                Err(Error::NegativeAmountErr)
            } else {
                Ok(f)
            }
        }
        Err(_) => Err(Error::InvalidAmountErr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_build_uri() {
        let mut params = HashMap::new();
        params.insert(
            String::from("somethingyoudontunderstan"),
            String::from("50"),
        );
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

    #[test]
    fn test_parse() {
        let p = parse(String::from("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=50&label=Luke-Jr&message=Donation for project&req-somethingelseyoudontget=999")).unwrap();
        assert_eq!(p.urn_scheme, String::from("bitcoin"));
        assert_eq!(
            p.address,
            String::from("175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W")
        );
        assert_eq!(p.amount, Some(50.0));
        assert_eq!(p.label, Some(String::from("Luke-Jr")));
        assert_eq!(p.message, Some(String::from("Donation for project")));
        assert_eq!(
            p.params
                .unwrap()
                .get("req-somethingelseyoudontget")
                .unwrap(),
            "999"
        );
    }
}
