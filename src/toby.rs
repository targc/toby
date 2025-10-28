use chrono::{DateTime, Utc};
use std::{future::Future, pin::Pin, sync::Arc};
use teloxide::prelude::*;

use super::command::{Command, parse_command};

#[derive(Debug)]
pub struct Msg {
    pub group_id: String,
    pub sender_username: Option<String>,
    pub ts: DateTime<Utc>,
    pub cmd: Command,
}

#[derive(Debug)]
pub struct ReplyMsg {
    pub text: String,
}

pub type HandlerFn = fn(&Msg) -> Pin<Box<dyn Future<Output = anyhow::Result<ReplyMsg>> + Send>>;

pub struct Toby {
    bot: Bot,
    handler: Arc<HandlerFn>,
}

impl Toby {
    pub fn new<TToken>(token: TToken, handler: HandlerFn) -> Self
    where
        TToken: Into<String>,
    {
        let bot = Bot::new(token);

        return Self {
            bot,
            handler: Arc::new(handler),
        };
    }

    pub async fn listen(self) {
        let hfn = self.handler.clone();

        teloxide::repl(self.bot, move |bot: Bot, msg: Message| {
            let hfn = hfn.clone();
            async move {
                dbg!(&msg);

                if let Some(text) = msg.text() {
                    dbg!(text);

                    if let Some(cmd) = parse_command(text) {
                        let msg_cmd = Msg {
                            group_id: msg.chat.id.to_string(),
                            sender_username: msg.from.as_ref().and_then(|a| a.username.clone()),
                            ts: msg.date,
                            cmd: cmd,
                        };

                        dbg!(&msg_cmd);

                        let reply = hfn(&msg_cmd).await;

                        match reply {
                            Ok(reply_msg) => {
                                dbg!(&reply_msg);
                                let _ = bot.send_message(msg.chat.id, reply_msg.text).await;
                            }
                            Err(_) => todo!(),
                        }
                    }
                }

                Ok(())
            }
        })
        .await;
    }
}
