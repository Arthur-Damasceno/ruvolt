# Ruvolt
`ruvolt` is an API library to interact with [Revolt Chat](https://revolt.chat) APIs and create bots.

## Getting started

### Installation
To use `ruvolt` we need [ruvolt](https://github.com/Arthur-Damasceno/ruvolt) crate, [async-trait](https://github.com/dtolnay/async-trait) crate and an asynchronous runtime, let's use the [tokio](https://github.com/tokio-rs/tokio).
Add this to your `Cargo.toml` *dependencies* section and run `cargo build` to compile dependencies.

```toml
ruvolt = "*"
async-trait = "*"
tokio = { version = "*", features = ["full"] }
```

### Example - Ping/Pong bot

```rust
use {
    async_trait::async_trait,
    ruvolt::{entities::Message, Client, Context, EventHandler, Result},
    std::env,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, cx: Context, msg: Message) {
        let content = msg.content.to_string();

        if content.as_str() == "!ping" {
            msg.reply(&cx, "Pong!").await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() -> Result {
    let token = env::var("TOKEN").unwrap();
    let client = Client::new(Handler).await?;

    if let Err(err) = client.listen(&token).await {
        eprintln!("{}", err);
    }

    Ok(())
}
```

### Documentation
*Coming soon*

## License
This project is under the [MIT](LICENSE) license.
