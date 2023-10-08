use crate::utils;
use lettre::message::{header::ContentType, Attachment, Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use std::fs;

pub fn send_email(name: &str, email_address: &str, body: &str) {
    let filename = "image".to_string();
    let filebody = fs::read(&utils::get_var("IMAGE_PATH")).unwrap();
    let attachment = Attachment::new_inline(filename.to_string())
        .body(filebody, ContentType::parse("image/jpg").unwrap());

    let email = Message::builder()
        .from(
            format!("Robodennis <{}>", utils::get_var("GMAIL_USERNAME"))
                .parse()
                .unwrap(),
        )
        .to(format!("{} <{}>", name, email_address).parse().unwrap())
        .subject("Rent Due")
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .content_type(ContentType::TEXT_PLAIN)
                        .body(body.to_string()),
                )
                .singlepart(attachment),
        )
        .unwrap();

    let creds = Credentials::new(
        utils::get_var("GMAIL_USERNAME"),
        utils::get_var("GMAIL_PASSWORD"),
    );

    // Open a connection to the server
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // send the email
    let result = mailer.send(&email);
    if result.is_ok() {
        println!("email sent successfully to {}.", name);
    } else {
        println!("Failed to send to {}: {:?}", email_address, result);
    }
}
