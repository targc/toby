use anyhow::anyhow;
use std::{future::Future, pin::Pin};
use tbot::{Msg, ReplyMsg, TBot};

fn handle_msg(_msg: &Msg) -> Pin<Box<dyn Future<Output = anyhow::Result<ReplyMsg>> + Send>> {
    Box::pin(async move { Err(anyhow!("todo")) })
}

#[tokio::main]
async fn main() {
    let token = "<TELEGRAM_TOKEN_HERE>";

    TBot::new(token, handle_msg).listen().await;
}
