use std::time::Duration;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::TlsParameters;
use lettre::{Message, SmtpTransport, Transport};

#[allow(non_snake_case)]
fn main() {
    let EMAIL_SMTP = std::env::var("EMAIL_SMTP").expect("EMAIL_SMTP must be set");
    let EMAIL_USERNAME = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME must be set");
    let EMAIL_PASSWORD = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set");

    println!("Sending email...");

    let email = Message::builder()
        .from(EMAIL_USERNAME.parse().unwrap())
        // .reply_to("Fausto Parisian <gina.koch@ethereal.email>".parse().unwrap())
        .to("foo@example.com".parse().unwrap())
        .subject("Happy new year")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy!"))
        .unwrap();

    let creds = Credentials::new(EMAIL_USERNAME.to_owned(), EMAIL_PASSWORD.to_owned());

    let tls_parameters = TlsParameters::builder(EMAIL_SMTP.to_string())
        .build()
        .unwrap();

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&EMAIL_SMTP)
        .unwrap()
        .credentials(creds)
        .tls(lettre::transport::smtp::client::Tls::Required(
            tls_parameters,
        ))
        .port(587)
        .timeout(Some(Duration::from_secs(60)))
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
