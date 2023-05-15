use bisky::atproto::ClientBuilder;
// use bisky::atproto::{Client, ClientBuilder, UserSession};
use bisky::bluesky::Bluesky;
use bisky::lexicon::app::bsky::feed::Post;
// use bisky::storage::{File, Storage as _};
// use clap::Parser;
use keyring::error::Error;
use keyring::Entry;
use std::env;
// use std::path::PathBuf;
// use std::sync::Arc;
use url::Url;

#[tokio::main]
async fn main() {
    // let args = Arguments::parse();
    // let storage = Arc::new(File::<UserSession>::new(args.storage));
    let mut client = ClientBuilder::default()
        .session(None)
        // .storage(None)
        .build()
        .unwrap();
    let service_url = Url::parse("https://bsky.social").unwrap();
    let username = get_credential(
        "SKIP_ENV_CHECK",
        "alan--bluesky--theidofalan--username",
        "alan",
    )
    .unwrap();
    let password = get_credential(
        "SKIP_ENV_CHECK",
        "alan--bluesky--theidofalan--api-key",
        "alan",
    )
    .unwrap();
    client
        .login(&service_url, username.as_str(), password.as_str())
        .await
        .unwrap();
    let mut bsky = Bluesky::new(client);
    println!(
        "{:#?}",
        bsky.me()
            .unwrap()
            .post(Post {
                text: "If you're seeing this, the bot code worked".to_string(),
                created_at: chrono::Utc::now(),
                rust_type: Some("app.bsky.feed.post".to_string()),
                embed: None,
                reply: None,
            })
            .await
            .unwrap()
    );
}

fn get_credential(envkey: &str, credkey: &str, creduser: &str) -> Result<String, Error> {
    if let Ok(value) = env::var(envkey) {
        Ok(value)
    } else {
        if let Ok(entry) = Entry::new(credkey, creduser) {
            if let Ok(value) = entry.get_password() {
                Ok(value)
            } else {
                Err(Error::NoEntry)
            }
        } else {
            Err(Error::NoEntry)
        }
    }
}
