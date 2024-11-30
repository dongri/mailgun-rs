#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};

use mailgun_rs::{EmailAddress, Mailgun, Message};

use dotenv::dotenv;
use std::env;

#[get("/")]
fn hello() -> String {
    let mut greeting = String::new();
    greeting.push_str("Hey!");
    greeting
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MailConfirmation<'a> {
    message: &'a str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Order<'a> {
    name: &'a str,
    recipient: &'a str,
}

#[post("/order/save", format = "application/json", data = "<order>")]
pub fn save_order(order: Json<Order<'_>>) -> Json<MailConfirmation> {
    // take the order and use the information to send an email
    let res = tokio::task::block_in_place(|| {
        send_mail_confirmation(&order);
    });
    println!("{:?}", res);

    let confirmation = MailConfirmation {
        message: "Mail sent successfully!",
    };
    Json(confirmation)
}

#[post("/order/save/async", format = "application/json", data = "<order>")]
pub async fn save_order_async(order: Json<Order<'_>>) -> Json<MailConfirmation> {
    // take the order and use the information to send an email
    send_mail_confirmation_async(&order).await;

    let confirmation = MailConfirmation {
        message: "Mail sent successfully!",
    };
    Json(confirmation)
}

fn send_mail_confirmation(order: &Json<Order<'_>>) {
    let api_key = env::var("MAILGUN_PRIVATE_API_KEY").expect("MAILGUN_PRIVATE_API_KEY not set").to_string();
    let domain = env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN not set").to_string();

    // send mail
    let recipient = EmailAddress::address(order.recipient);
    let message = Message {
        to: vec![recipient],
        subject: String::from("Order Confirmation"),
        html: String::from(format!("<b>Order Name: {}</b>", order.name)),
        ..Default::default()
    };

    let client = Mailgun {
        api_key: String::from(api_key),
        domain: String::from(domain),
    };
    let sender = EmailAddress::name_address("no-reply", "no-reply@hackerth.com");

    match client.send(&sender, message) {
        Ok(_) => {
            println!("successful");
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

async fn send_mail_confirmation_async(order: &Json<Order<'_>>) {
    let api_key = env::var("MAILGUN_PRIVATE_API_KEY").expect("MAILGUN_PRIVATE_API_KEY not set").to_string();
    let domain = env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN not set").to_string();

    // send mail
    let recipient = EmailAddress::address(order.recipient);
    let message = Message {
        to: vec![recipient],
        subject: String::from("Order Confirmation Async"),
        html: String::from(format!("<b>Order Name: {}</b>", order.name)),
        ..Default::default()
    };

    let client = Mailgun {
        api_key: String::from(api_key),
        domain: String::from(domain),
    };
    let sender = EmailAddress::name_address("no-reply", "no-reply@hackerth.com");

    match client.async_send(&sender, message).await {
        Ok(_) => {
            println!("successful");
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![hello])
        .mount("/", routes![save_order, save_order_async])
}
