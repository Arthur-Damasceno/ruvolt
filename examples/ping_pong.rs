use {
    async_trait::async_trait,
    ruvolt::{
        models::{events::ReadyEvent, Message, User},
        Client, Context, EventHandler, Result,
    },
    std::{env, time::Instant},
    tokio::time::{sleep, Duration},
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(cx: Context, _: ReadyEvent) {
        if let Ok(User { username, .. }) = cx.user().await {
            println!("@{username} is ready!");
        }
    }

    async fn message(cx: Context, msg: Message) {
        let content = msg.content.to_string();

        if content == "!ping" {
            let start = Instant::now();

            if let Ok(mut msg) = msg.reply(&cx, "Pong!", true).await {
                let delta_latency = (Instant::now() - start).as_millis();
                let bonfire_latency = cx.latency().await.as_millis();

                let content = format!(
                    "### Pong!\
                    \n#### The Delta API latency is `{delta_latency}ms`\
                    \n#### The Bonfire API latency is `{bonfire_latency}ms`"
                );

                msg.edit(&cx, content).await.ok();

                sleep(Duration::from_secs(5)).await;

                msg.delete(&cx).await.ok();
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result {
    let token = env::var("TOKEN").unwrap();
    let mut client = Client::new(token).await?;

    client.listen::<Handler>().await
}
