# rust-bip21

[![test](https://github.com/yassun/rust-bip21/actions/workflows/test.yaml/badge.svg)](https://github.com/yassun/rust-bip21/actions/workflows/test.yaml)

rust-bip21 is an open source library to handle the URI based on the [BIP-21](https://github.com/bitcoin/bips/blob/master/bip-0021.mediawiki) standard.

# Usage

Parse the URI `bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=50&label=Luke-Jr&message=Donation for project&req-somethingelseyoudontget=999`.

```Rust
u := bip21::parse(String::from("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=50&label=Luke-Jr&message=Donation for project&req-somethingelseyoudontget=999")).unwrap();

// UriResources { urn_scheme: "bitcoin", address: "175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W", amount: Some(50.0), label: Some("Luke-Jr"), message: Some("Donation for project"), params: Some({"req-somethingelseyoudontget": "999"}) }
println!("{:?}", u);
```

Build the URI `bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=20.3&label=Luke-Jr`

```Rust
let u = bip21::UriResources::new(
    String::from("bitcoin"),
    String::from("175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W"),
    Some(100.0),
    Some(String::from("Luke-Jr")),
    Some(String::from("message")),
    Some(HashMap::new()),
);

let uri = u.build_uri().unwrap();

// bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=20.3&label=Luke-Jr
println!("{:?}", uri);
```

