use lettre::address::AddressError as LettreAddressError;
use lettre::error::Error as LettreError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::transport::smtp::Error as LettreSmtpError;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use serde::{Deserialize, Serialize};
use std::fmt;

use base64::prelude::*;

#[derive(Debug)]
pub enum MailerError {
    Email(LettreError),
    Address(LettreAddressError),
    SmtpTransport(LettreSmtpError),
    Base64Decode(base64::DecodeError),
    InvalidContentType(String),
}

impl std::error::Error for MailerError {}

impl fmt::Display for MailerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MailerError::Email(err) => write!(f, "{err}"),
            MailerError::Address(err) => write!(f, "{err}"),
            MailerError::SmtpTransport(err) => write!(f, "{err}"),
            MailerError::Base64Decode(err) => write!(f, "base64 decode error: {err}"),
            MailerError::InvalidContentType(ct) => write!(f, "invalid content type: {ct}"),
        }
    }
}

impl From<base64::DecodeError> for MailerError {
    fn from(error: base64::DecodeError) -> Self {
        MailerError::Base64Decode(error)
    }
}

impl From<LettreError> for MailerError {
    fn from(error: LettreError) -> Self {
        MailerError::Email(error)
    }
}

impl From<LettreAddressError> for MailerError {
    fn from(error: LettreAddressError) -> Self {
        MailerError::Address(error)
    }
}

impl From<LettreSmtpError> for MailerError {
    fn from(error: LettreSmtpError) -> Self {
        MailerError::SmtpTransport(error)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AttachmentData {
    pub filename: String,
    pub content: String,
    pub mime_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MailOptions {
    pub from: String,
    pub reply_to: String,
    pub to: String,
    pub subject: String,
    pub body: String,
    #[serde(default)]
    pub attachments: Vec<AttachmentData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmtpCredentials {
    pub username: String,
    pub password: String,
}

impl From<SmtpCredentials> for Credentials {
    fn from(credentials: SmtpCredentials) -> Self {
        Credentials::new(credentials.username, credentials.password)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmtpConnectionOptions {
    pub host: String,
    pub credentials: SmtpCredentials,
}

pub fn parse_mail_options(options: MailOptions) -> Result<Message, MailerError> {
    let builder = Message::builder()
        .from(options.from.parse()?)
        .reply_to(options.reply_to.parse()?)
        .to(options.to.parse()?)
        .subject(options.subject);

    if options.attachments.is_empty() {
        Ok(builder.body(options.body)?)
    } else {
        let mut multipart = MultiPart::mixed().singlepart(SinglePart::plain(options.body));

        for attachment in options.attachments {
            let decoded = BASE64_STANDARD.decode(&attachment.content)?;
            let content_type: ContentType = attachment
                .mime_type
                .parse()
                .map_err(|_| MailerError::InvalidContentType(attachment.mime_type))?;
            let part = Attachment::new(attachment.filename).body(decoded, content_type);
            multipart = multipart.singlepart(part);
        }

        Ok(builder.multipart(multipart)?)
    }
}

pub async fn send_mail_smtp(
    message: Message,
    connection_options: SmtpConnectionOptions,
) -> Result<Response, MailerError> {
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&connection_options.host)?
            .credentials(connection_options.credentials.into())
            .build();

    Ok(mailer.send(message).await?)
}
