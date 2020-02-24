# mailgun-rs

An unofficial client library for the Mailgun API

```toml
# Cargo.toml
...
[dependencies]
mailgun-rs = "0.1.3"
...
```

### Examples

```rust
extern crate mailgun_rs;

use mailgun_rs::{Mailgun, EmailAddress, Message};
use std::error::Error;

fn main() {
    let domain = "hackerth.com";
    let key = "key-xxxxxx";
    let recipient = "dongrify@gmail.com";
    let recipient = EmailAddress::address(&recipient);
    let message = Message {
        to: vec![recipient],
        subject: String::from("mailgun-rs"),
        html: String::from("<h1>hello from mailgun</h1>"),
        ..Default::default()
    };

    let client = Mailgun{api_key: String::from(key), domain: String::from(domain), message: message};
    let sender = EmailAddress::name_address("no-reply", "no-reply@hackerth.com");
    
    match client.send(&sender) {
      Ok(_) => {
        println!("successful");
      }
      Err(err) => {
        println!("{}", err.description());
      }
    }
}
```
