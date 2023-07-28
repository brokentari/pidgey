use crate::domain::SubscriberEmail;
use reqwest::Client;
use secrecy::{ExposeSecret, Secret};

pub struct EmailClient {
    http_client: Client,
    base_url: reqwest::Url,
    sender: SubscriberEmail,
    authorization_token: Secret<String>,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: Secret<String>,
    ) -> Result<EmailClient, String> {
        let url = reqwest::Url::parse(base_url.as_str()).expect("failed to parse url");
        Ok(Self {
            http_client: Client::new(),
            base_url: url,
            sender,
            authorization_token,
        })
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = self
            .base_url
            .join("/v3/mail/send")
            .expect("failed to join url");
        let request_body = SendEmailRequest {
            personalizations: Personalizations {
                to: vec![EmailAddress {
                    email: recipient.as_ref(),
                }],
            },
            from: EmailAddress {
                email: self.sender.as_ref(),
            },
            subject,
            content: vec![
                EmailContent {
                    r#type: "text/plain",
                    value: text_content,
                },
                EmailContent {
                    r#type: "text/html",
                    value: html_content,
                },
            ],
        };
        self.http_client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.authorization_token.expose_secret()),
            )
            .json(&request_body)
            .send()
            .await?;

        Ok(())
    }
}

#[derive(serde::Serialize)]
struct SendEmailRequest<'a> {
    personalizations: Personalizations<'a>,
    from: EmailAddress<'a>,
    subject: &'a str,
    content: Vec<EmailContent<'a>>,
}

#[derive(serde::Serialize)]
struct Personalizations<'a> {
    to: Vec<EmailAddress<'a>>,
}

#[derive(serde::Serialize)]
struct EmailAddress<'a> {
    email: &'a str,
}

#[derive(serde::Serialize)]
struct EmailContent<'a> {
    r#type: &'a str,
    value: &'a str,
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use secrecy::Secret;
    use wiremock::matchers::{header, header_exists, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client =
            EmailClient::new(mock_server.uri(), sender, Secret::new(Faker.fake())).unwrap();
        Mock::given(header_exists("Authorization"))
            .and(header("Content-Type", "application/json"))
            .and(path("/v3/mail/send"))
            .and(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;
        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;
    }
}
