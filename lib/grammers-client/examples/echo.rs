//! Example to echo user text messages. Runnable as:
//!
//! ```sh
//! cargo run --example echo -- API_ID API_HASH BOT_TOKEN
//! ```

use async_std::task;
use grammers_client::ext::{MessageExt, UpdateExt};
use grammers_client::{AuthorizationError, Client, Config};
use grammers_session::Session;
use log;
use simple_logger;
use std::env;

async fn async_main() -> Result<(), AuthorizationError> {
    simple_logger::init_with_level(log::Level::Debug).expect("failed to setup logging");

    let mut args = env::args();

    let _path = args.next();
    let api_id = args
        .next()
        .expect("api_id missing")
        .parse()
        .expect("api_id invalid");
    let api_hash = args.next().expect("api_hash missing");
    let token = args.next().expect("token missing");

    println!("Connecting to Telegram...");
    let mut client = Client::connect(Config {
        session: Session::load_or_create("echo.session")?,
        api_id,
        api_hash: api_hash.clone(),
        params: Default::default(),
    })
    .await?;
    println!("Connected!");

    if !client.is_authorized().await? {
        println!("Signing in...");
        client.bot_sign_in(&token, api_id, &api_hash).await?;
        println!("Signed in!");
    }

    println!("Waiting for messages...");
    while let Some((updates, entity_set)) = client.next_updates().await {
        for update in updates {
            if let Some(message) = update.message() {
                let peer = entity_set
                    .get(&message.chat())
                    .expect("failed to find entity");

                println!("Responding to {}", peer.name());
                client
                    .send_message(peer.to_input_peer(), message.message.as_str().into())
                    .await?;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), AuthorizationError> {
    task::block_on(async_main())
}
