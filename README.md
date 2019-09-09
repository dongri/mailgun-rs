# mailgun-rs

An unofficial client library for the Mailgun API

### examples
```rust
extern crate mailgun_rs;

use mailgun_rs::{Mailgun, EmailAddress, Message};

fn mailgun() {
    let domain = "domain.com";
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
    
    let res = client.send(&sender);
    println!("{:?}", res);
}
```
