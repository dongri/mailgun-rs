use std::{env, fs, path::Path};

use mailgun_rs::{Attachment, AttachmentType, EmailAddress, Mailgun, MailgunRegion, Message};
use std::collections::HashMap;

fn main() {
    let domain = &env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN not set");
    let key = &env::var("MAILGUN_API_KEY").expect("MAILGUN_API_KEY not set");
    let recipient = "dongrium@gmail.com";

    send_html(recipient, key, domain);
    send_template(recipient, key, domain);
    send_with_attachment(recipient, key, domain);
    send_with_inline_attachment(recipient, key, domain);
    println!("All emails sent successfully.");
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
    let sender = EmailAddress::name_address("no-reply", "no-reply@huatuo.xyz");

    match client.send(MailgunRegion::US, &sender, message, None) {
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
    template_vars.insert(String::from("name"), String::from("Dongri Jin"));
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
    let sender = EmailAddress::name_address("no-reply", "no-reply@huatuo.xyz");

    match client.send(MailgunRegion::US, &sender, message, None) {
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
        let file_name = format!("examples/attachments/sample-{item}.txt");
        let absolute_path =
            fs::canonicalize(Path::new(&file_name)).expect("cannot get absolute path");

        attachments.push(
            Attachment::builder()
                .path(absolute_path.to_string_lossy().to_string())
                .attachment_type(AttachmentType::Attachment)
                .build(),
        );
    }

    let client = Mailgun {
        api_key: String::from(key),
        domain: String::from(domain),
    };

    let sender = EmailAddress::name_address("no-reply", "no-reply@huatuo.xyz");

    match client.send(MailgunRegion::US, &sender, message, Some(attachments)) {
        Ok(_) => {
            println!("successful");
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

fn send_with_inline_attachment(recipient: &str, key: &str, domain: &str) {
    let recipient = EmailAddress::address(recipient);
    let message = Message {
        to: vec![recipient],
        subject: String::from("mailgun-rs"),
        html: String::from(
            "<h1>hello from mailgun with inline attachment</h1><img src=\"cid:inline.png\">",
        ),
        ..Default::default()
    };

    let mut attachments = Vec::new();
    let file_name = "examples/attachments/sushi.png";
    let absolute_path = fs::canonicalize(Path::new(&file_name)).expect("cannot get absolute path");

    // Create an inline attachment
    attachments.push(
        Attachment::builder()
            .path(absolute_path.to_string_lossy().to_string())
            .attachment_type(AttachmentType::Inline)
            .build(),
    );

    let client = Mailgun {
        api_key: String::from(key),
        domain: String::from(domain),
    };

    let sender = EmailAddress::name_address("no-reply", "no-reply@huatuo.xyz");

    match client.send(MailgunRegion::US, &sender, message, Some(attachments)) {
        Ok(_) => {
            println!("successful");
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

// MAILGUN_DOMAIN=xxx MAILGUN_API_KEY=xxx-xxx-xxx cargo run --package mailgun-rs --example basic
