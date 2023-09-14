use std::env;

use mailgun_rs::{EmailAddress, Mailgun, MailgunRegion, Message};
use std::collections::HashMap;

fn main() {
    let domain = &env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN not set");
    let key = &env::var("MAILGUN_PRIVATE_API_KEY").expect("MAILGUN_PRIVATE_API_KEY not set");
    let recipient = "dongrify@gmail.com";

    send_html(recipient, key, domain);
    send_template(recipient, key, domain);
}

fn send_html(recipient: &str, key: &str, domain: &str) {
    let recipient = EmailAddress::address(recipient);
    let message = Message {
        to: vec![recipient],
        subject: String::from("mailgun-rs"),
        html: String::from("<h1>hello from mailgun</h1>"),
        ..Default::default()
    };
    let client = Mailgun {
        api_key: String::from(key),
        domain: String::from(domain),
        message,
    };
    let sender = EmailAddress::name_address("no-reply", "no-reply@hackerth.com");
    match client.send(MailgunRegion::US, &sender) {
        Ok(_) => {
            println!("successful");
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

fn send_template(recipient: &str, key: &str, domain: &str) {
    let mut template_vars = HashMap::new();
    template_vars.insert(String::from("firstname"), String::from("Dongri"));
    let recipient = EmailAddress::address(recipient);
    let message = Message {
        to: vec![recipient],
        subject: String::from("mailgun-rs"),
        template: String::from("template-1"),
        template_vars,
        ..Default::default()
    };
    let client = Mailgun {
        api_key: String::from(key),
        domain: String::from(domain),
        message,
    };
    let sender = EmailAddress::name_address("no-reply", "no-reply@hackerth.com");
    match client.send(MailgunRegion::US, &sender) {
        Ok(_) => {
            println!("successful");
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

// MAILGUN_DOMAIN=xxx MAILGUN_PRIVATE_API_KEY=xxx-xxx-xxx cargo run --package mailgun-rs --example basic
