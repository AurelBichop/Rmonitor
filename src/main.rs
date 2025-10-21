use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use dotenv::dotenv;
use std::env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prefix = "https://".to_string();

    // List of sites and search words
    let site_web: [&str; 3] = ["url_web1","url_web2", "..."];
    let pattern_searh: [&str; 3] = ["word1","word2", "..."];
    
    for i in 0..2{

        let resp = reqwest::get(prefix.clone() + site_web[i])
        .await?
        .text()
        //.json::<HashMap<String, String>>()
        .await?;
    
        if !resp.contains(pattern_searh[i]) 
        {
            send_mail(site_web[i]);
        };

    }
    
    Ok(())
}


fn send_mail(name_site: &str) {
    dotenv().ok();

    let smtp_server: String = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
    let login: String = env::var("LOGIN").expect("LOGIN must be set");
    let password: String = env::var("PASSWORD").expect("PASSWORD must be set");

    let sender: String = env::var("SENDER").expect("SENDER must be set");
    let recipient: String = env::var("RECIPIENT").expect("RECIPIENT must be set");
    let subject = env::var("SUBJECT").expect("SUBJECT must be set");
    let mess: String = env::var("ERROR_MESSAGE").expect("ERROR_MESSAGE must be set");

    
    let email = Message::builder()
        .from(Mailbox::new(Some(sender.to_owned()), sender.parse().unwrap()))
        .reply_to(Mailbox::new(Some(sender.to_owned()), sender.parse().unwrap()))
        .to(Mailbox::new(Some(recipient.to_owned()), recipient.parse().unwrap()))
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(mess + name_site)
        .unwrap();

    let creds = Credentials::new(login.to_owned(), password.to_owned());

    // Open a remote connection
    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}