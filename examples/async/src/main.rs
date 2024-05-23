use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use mailgun_rs::{EmailAddress, Mailgun, Message, MailgunRegion};

static MAILGUN_CLIENT: OnceCell<Mutex<Mailgun>> = OnceCell::new();

#[tokio::main]
async fn main() {
    dotenv().ok();

    let domain = &env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN not set");
    let key = &env::var("MAILGUN_API_KEY").expect("MAILGUN_API_KEY not set");
    let recipient = "dongrium@gmail.com";

    initialize_mailgun(key, domain);

    send_html(recipient).await;
    send_template(recipient, key, domain).await;
}

fn initialize_mailgun(api_key: &str, domain: &str) {
    let mailgun_client = Mailgun {
        api_key: api_key.to_string(),
        domain: domain.to_string(),
        message: Message {
            ..Default::default()
        },
    };

    MAILGUN_CLIENT.set(Mutex::new(mailgun_client)).expect("Mailgun client can only be initialized once");
}

async fn send_html(recipient: &str) {
    let recipient = EmailAddress::address(recipient);
    let message = Message {
        to: vec![recipient],
        subject: String::from("mailgun-rs"),
        html: String::from("<h1>hello from mailgun</h1>"),
        ..Default::default()
    };

    let sender = EmailAddress::name_address("no-reply", "no-reply@hackerth.com");

    if let Some(client) = MAILGUN_CLIENT.get() {
        let mut mailgun_client = client.lock().unwrap();
        mailgun_client.message = message.clone();

        match mailgun_client.async_send(MailgunRegion::US, &sender).await {
            Ok(_) => {
                println!("successful");
            }
            Err(err) => {
                println!("Error: {err}");
            }
        }
    } else {
        println!("Mailgun client is not initialized");
    }
}

async fn send_template(recipient: &str, key: &str, domain: &str) {
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
    match client.async_send(MailgunRegion::US, &sender).await {
        Ok(_) => {
            println!("successful");
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}
