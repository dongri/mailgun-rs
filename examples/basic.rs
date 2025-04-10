use std::{env, fs, path::Path};

use mailgun_rs::{EmailAddress, Mailgun, MailgunRegion, Message};
use std::collections::HashMap;

fn main() {
    let domain = &env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN not set");
    let key = &env::var("MAILGUN_API_KEY").expect("MAILGUN_API_KEY not set");
    let recipient = "dongrium@gmail.com";

    send_html(recipient, key, domain);
    send_template(recipient, key, domain);
    send_with_attachment(recipient, key, domain);
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
    };
    let sender = EmailAddress::name_address("no-reply", "no-reply@hackerth.com");
    match client.send(MailgunRegion::US, &sender, message, Vec::new()) {
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
    };
    let sender = EmailAddress::name_address("no-reply", "no-reply@hackerth.com");
    match client.send(MailgunRegion::US, &sender, message, Vec::new()) {
        Ok(_) => {
            println!("successful");
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

fn send_with_attachment(recipient: &str, key: &str, domain: &str) {
    let recipient = EmailAddress::address(recipient);
    let message = Message {
        to: vec![recipient],
        subject: String::from("mailgun-rs"),
        html: String::from("<h1>hello from mailgun with attachment</h1>"),
        ..Default::default()
    };

    let mut attachments = Vec::new();
    for item in ["file-1", "file-2"] {
        let file_name = format!("sample-{item}.txt");
        let file_content = format!("hello from sample {item}");
        fs::write(&file_name, &file_content).expect("cannot write file");

        let absolute_path =
            fs::canonicalize(Path::new(&file_name)).expect("cannot get absolute path");

        attachments.push(absolute_path.to_string_lossy().to_string());
    }

    let client = Mailgun {
        api_key: String::from(key),
        domain: String::from(domain),
    };

    let sender = EmailAddress::name_address("no-reply", "no-reply@hackerth.com");

    match client.send(MailgunRegion::US, &sender, message, attachments) {
        Ok(_) => {
            println!("successful");
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

// MAILGUN_DOMAIN=xxx MAILGUN_API_KEY=xxx-xxx-xxx cargo run --package mailgun-rs --example basic
