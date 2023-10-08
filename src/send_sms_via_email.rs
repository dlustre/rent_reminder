use crate::utils;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_sms_via_email(name: &str, number: &str, body: &str, gateway: &Option<String>) {
    match gateway {
        Some(email_gateway) => {
            let email = Message::builder()
                .from(
                    format!("RoboDennis <{}>", utils::get_var("GMAIL_USERNAME"))
                        .parse()
                        .unwrap(),
                )
                .to(format!("{} <{}@{}>", name, number, email_gateway)
                    .parse()
                    .unwrap())
                .subject("Rent Due")
                .body(body.to_string())
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
                println!("SMS via email sent successfully to {}.", name);
            } else {
                println!("Failed to send to {:?}: {:?}", email, result);
            }
        }
        None => println!("No suitable gateway for given carrier"),
    }
}
