use {
    async_trait::async_trait,
    ruvolt::{
        error::Error,
        models::{events::ReadyEvent, Message},
        Client, Context, EventHandler, Result,
    },
    std::{env, time::Instant},
    tokio::time::{sleep, Duration},
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn error(&self, err: Error) {
        eprintln!("{}", err);
    }

    async fn ready(&self, cx: Context, _: ReadyEvent) {
        println!("{} is ready!", cx.user.username);
    }

    async fn message(&self, cx: Context, msg: Message) {
        if msg.author_id == cx.user.id {
            return;
        }

        let content = msg.content.to_string();

        if content == "!ping" {
            let now = Instant::now();
            let mut msg = msg.send_in_channel(&cx, "Pong!").await.unwrap();

            let latency = (Instant::now() - now).subsec_millis();
            let content = format!("Pong! The API latency is {}ms", latency);

            msg.edit(&cx, &content).await.unwrap();

            sleep(Duration::from_secs(3)).await;
            msg.delete(&cx).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() -> Result {
    let token = env::var("TOKEN").unwrap();
    let mut client = Client::new(Handler, token).await?;

    if let Err(err) = client.listen().await {
        eprintln!("{}", err);
    }

    Ok(())
}
