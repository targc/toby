# Toby

A simple, opinionated Telegram bot library built on top of [teloxide](https://github.com/teloxide/teloxide), providing an easy-to-use interface for building Telegram bots with structured command parsing.

## Features

- **Simple API**: Minimal boilerplate to get your bot running
- **Structured Command Parsing**: Parse commands with both positional arguments and key-value pairs
- **Async Handler**: Modern async/await support with tokio
- **Type-safe Message Handling**: Structured message and reply types

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toby = { path = "." }  # Or publish to crates.io and use version
tokio = { version = "1.8", features = ["rt-multi-thread", "macros"] }
anyhow = "1.0"
```

## Quick Start

```rust
use anyhow::anyhow;
use std::{future::Future, pin::Pin};
use toby::{Msg, ReplyMsg, Toby};

fn handle_msg(msg: &Msg) -> Pin<Box<dyn Future<Output = anyhow::Result<ReplyMsg>> + Send>> {
    Box::pin(async move {
        // Handle different commands
        match msg.cmd.name.as_str() {
            "hello" => {
                Ok(ReplyMsg {
                    text: format!("Hello, {}!", msg.sender_username.as_deref().unwrap_or("stranger"))
                })
            }
            "echo" => {
                let text = msg.cmd.short_args.join(" ");
                Ok(ReplyMsg { text })
            }
            _ => Err(anyhow!("Unknown command"))
        }
    })
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN not set");

    Toby::new(token, handle_msg).listen().await;
}
```

## Command Format

Toby supports a special command format that allows both short arguments and key-value pairs:

### Basic Command with Short Arguments
```
/command arg1 arg2 arg3
```

### Command with Key-Value Pairs
```
/command short_arg1 short_arg2
- key1: value1
- key2: value2
- key3: value3
```

### Examples

**Simple command:**
```
/hello world
```

**Complex command with metadata:**
```
/create task123
- title: Buy groceries
- priority: high
- due: 2024-01-15
```

## API Reference

### `Toby`

The main bot struct.

#### `Toby::new(token, handler)`
Creates a new bot instance.

- `token`: Your Telegram bot token (get it from [@BotFather](https://t.me/botfather))
- `handler`: A function that takes `&Msg` and returns a `Future<Output = anyhow::Result<ReplyMsg>>`

#### `Toby::listen()`
Starts the bot and listens for incoming messages.

### `Msg`

Represents an incoming message.

```rust
pub struct Msg {
    pub group_id: String,              // Chat/group ID
    pub sender_username: Option<String>, // Sender's username (if available)
    pub ts: DateTime<Utc>,              // Message timestamp
    pub cmd: Command,                   // Parsed command
}
```

### `Command`

Represents a parsed command.

```rust
pub struct Command {
    pub name: String,                   // Command name (without /)
    pub short_args: Vec<String>,        // Positional arguments
    pub kv: HashMap<String, String>,    // Key-value pairs
}
```

### `ReplyMsg`

Represents a reply message to send back.

```rust
pub struct ReplyMsg {
    pub text: String,  // Reply text
}
```

## Examples

See the `examples/` directory for complete examples:

```bash
cargo run --example simple_usage
```

## Environment Variables

- `TELEGRAM_BOT_TOKEN`: Your bot token from BotFather (recommended for production use)

## Development

### Building

```bash
cargo build
```

### Running Examples

```bash
# Replace <TOKEN> with your actual bot token
TELEGRAM_BOT_TOKEN=<TOKEN> cargo run --example simple_usage
```

## Dependencies

- [teloxide](https://github.com/teloxide/teloxide) - Telegram bot framework
- [tokio](https://tokio.rs/) - Async runtime
- [regex](https://docs.rs/regex/) - Command parsing
- [chrono](https://docs.rs/chrono/) - Date and time handling
- [anyhow](https://docs.rs/anyhow/) - Error handling

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
