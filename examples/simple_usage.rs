use anyhow::anyhow;
use std::{future::Future, pin::Pin};
use toby::{Msg, ReplyMsg, Toby};

fn handle_msg(_msg: &Msg) -> Pin<Box<dyn Future<Output = anyhow::Result<ReplyMsg>> + Send>> {
    Box::pin(async move { Err(anyhow!("todo")) })
}

#[tokio::main]
async fn main() {
    let token = "<TELEGRAM_TOKEN_HERE>";

    Toby::new(token, handle_msg).listen().await;
}
