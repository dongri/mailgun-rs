use reqwest::Error as ReqError;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use typed_builder::TypedBuilder;

const MESSAGES_ENDPOINT: &str = "messages";

pub enum MailgunRegion {
    US,
    EU,
}

fn get_base_url(region: MailgunRegion) -> &'static str {
    match region {
        MailgunRegion::US => "https://api.mailgun.net/v3",
        MailgunRegion::EU => "https://api.eu.mailgun.net/v3",
    }
}

#[derive(Default, Debug)]
pub struct Mailgun {
    pub api_key: String,
    pub domain: String,
}

pub type SendResult<T> = Result<T, ReqError>;

#[derive(Deserialize, Debug, PartialEq)]
pub struct SendResponse {
    pub message: String,
    pub id: String,
}

impl Mailgun {
    pub fn send(
        &self,
        region: MailgunRegion,
        sender: &EmailAddress,
        message: Message,
    ) -> SendResult<SendResponse> {
        let client = reqwest::blocking::Client::new();
        let mut params = message.params();
        params.insert("from".to_string(), sender.to_string());
        let url = format!(
            "{}/{}/{}",
            get_base_url(region),
            self.domain,
            MESSAGES_ENDPOINT
        );

        let res = client
            .post(url)
            .basic_auth("api", Some(self.api_key.clone()))
            .form(&params)
            .send()?
            .error_for_status()?;

        let parsed: SendResponse = res.json()?;
        Ok(parsed)
    }

    pub async fn async_send(
        &self,
        region: MailgunRegion,
        sender: &EmailAddress,
        message: Message,
    ) -> SendResult<SendResponse> {
        let client = reqwest::Client::new();
        let mut params = message.params();
        params.insert("from".to_string(), sender.to_string());
        let url = format!(
            "{}/{}/{}",
            get_base_url(region),
            self.domain,
            MESSAGES_ENDPOINT
        );

        let res = client
            .post(url)
            .basic_auth("api", Some(self.api_key.clone()))
            .form(&params)
            .send()
            .await?
            .error_for_status()?;

        let parsed: SendResponse = res.json().await?;
        Ok(parsed)
    }
}

#[derive(TypedBuilder, Default, Debug, PartialEq, Eq, Clone)]
pub struct Message {
    #[builder(setter(into))]
    pub to: Vec<EmailAddress>,
    #[builder(default, setter(into))]
    pub cc: Vec<EmailAddress>,
    #[builder(default, setter(into))]
    pub bcc: Vec<EmailAddress>,
    #[builder(setter(into))]
    pub subject: String,
    #[builder(default, setter(into))]
    pub text: String,
    #[builder(default, setter(into))]
    pub html: String,
    #[builder(default, setter(into))]
    pub template: String,
    #[builder(default)]
    pub template_vars: HashMap<String, String>,
    #[builder(default)]
    pub template_json: Option<serde_json::Value>,
}

impl Message {
    fn params(self) -> HashMap<String, String> {
        let mut params = HashMap::new();

        Message::add_recipients("to", self.to, &mut params);
        Message::add_recipients("cc", self.cc, &mut params);
        Message::add_recipients("bcc", self.bcc, &mut params);

        params.insert(String::from("subject"), self.subject);

        params.insert(String::from("text"), self.text);
        params.insert(String::from("html"), self.html);

        // add template
        if !self.template.is_empty() {
            params.insert(String::from("template"), self.template);
            if let Some(template_json) = self.template_json {
                params.insert(
                    String::from("h:X-Mailgun-Variables"),
                    serde_json::to_string(&template_json).unwrap(),
                );
            } else {
                params.insert(
                    String::from("h:X-Mailgun-Variables"),
                    serde_json::to_string(&self.template_vars).unwrap(),
                );
            }
        }

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

#[derive(TypedBuilder, Debug, PartialEq, Eq, Clone)]
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
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.name {
            Some(ref name) => write!(f, "{} <{}>", name, self.address),
            None => write!(f, "{}", self.address),
        }
    }
}

impl From<&str> for EmailAddress {
    fn from(address: &str) -> Self {
        EmailAddress::address(address)
    }
}

impl From<(&str, &str)> for EmailAddress {
    fn from((name, address): (&str, &str)) -> Self {
        EmailAddress::name_address(name, address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_builder_should_work() {
        let message = Message::builder()
            .to(vec!["example@example.com".into()])
            .cc(vec![("Eren", "eren@redmc.me").into()])
            .text("")
            .html("<h1>Hello</h1>")
            .subject("Hello")
            .template("template")
            .template_vars([("name".into(), "value".into())].iter().cloned().collect())
            .build();
        assert_eq!(
            message,
            Message {
                to: vec![EmailAddress {
                    name: None,
                    address: "example@example.com".to_string()
                }],
                cc: vec![EmailAddress {
                    name: Some("Eren".to_string()),
                    address: "eren@redmc.me".to_string()
                }],
                bcc: vec![],
                subject: "Hello".to_string(),
                text: "".to_string(),
                html: "<h1>Hello</h1>".to_string(),
                template: "template".to_string(),
                template_vars: [("name".into(), "value".into())].iter().cloned().collect(),
                template_json: None,
            }
        );
    }
}
