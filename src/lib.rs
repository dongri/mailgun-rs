use reqwest;
use reqwest::Error as ReqError;
use serde::Deserialize;
use std::collections::HashMap;

const MAILGUN_API: &str = "https://api.mailgun.net/v3";
const MESSAGES_ENDPOINT: &str = "messages";

#[derive(Default)]
pub struct Mailgun {
    pub api_key: String,
    pub domain: String,
    pub message: Message,
}

pub type SendResult<T> = Result<T, ReqError>;

#[derive(Deserialize, Debug, PartialEq)]
pub struct SendResponse {
    pub message: String,
    pub id: String,
}

impl Mailgun {
    pub fn send(self, sender: &EmailAddress) -> SendResult<SendResponse> {
        let client = reqwest::Client::new();
        let mut params = self.message.to_params();
        params.insert("from".to_string(), sender.to_string());
        let url = format!("{}/{}/{}", MAILGUN_API, self.domain, MESSAGES_ENDPOINT);

        let mut res = client
            .post(&url)
            .basic_auth("api", Some(self.api_key.clone()))
            .form(&params)
            .send()?
            .error_for_status()?;

        let parsed: SendResponse = res.json()?;
        Ok(parsed)
    }
}

#[derive(Default)]
pub struct Message {
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub subject: String,
    pub text: String,
    pub html: String,
}

impl Message {
    fn to_params(self) -> HashMap<String, String> {
        let mut params = HashMap::new();

        Message::add_recipients("to", self.to, &mut params);
        Message::add_recipients("cc", self.cc, &mut params);
        Message::add_recipients("bcc", self.bcc, &mut params);

        params.insert(String::from("subject"), self.subject);

        params.insert(String::from("text"), self.text);
        params.insert(String::from("html"), self.html);

        params
    }

    fn add_recipients(
        field: &str,
        addresses: Vec<EmailAddress>,
        params: &mut HashMap<String, String>,
    ) {
        if !addresses.is_empty() {
            let joined = addresses
                .iter()
                .map(EmailAddress::to_string)
                .collect::<Vec<String>>()
                .join(",");
            params.insert(field.to_owned(), joined);
        }
    }
}

pub struct EmailAddress {
    name: Option<String>,
    address: String,
}

impl EmailAddress {
    pub fn address(address: &str) -> Self {
        EmailAddress {
            name: None,
            address: address.to_string(),
        }
    }

    pub fn name_address(name: &str, address: &str) -> Self {
        EmailAddress {
            name: Some(name.to_string()),
            address: address.to_string(),
        }
    }

    pub fn email(&self) -> &str {
        &self.address
    }

    pub fn to_string(&self) -> String {
        match self.name {
            Some(ref name) => format!("{} <{}>", name, self.address),
            None => self.address.clone(),
        }
    }
}
