use {
    async_trait::async_trait,
    ruvolt::{
        models::{events::ReadyEvent, Message, Presence, UserStatus},
        Client, Context, EventHandler, Result,
    },
    std::env,
};

#[derive(Clone, Default)]
struct Counter(u32);

impl Counter {
    fn increment(&mut self) {
        self.0 += 1;
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, cx: Context, _: ReadyEvent) {
        cx.state.insert(Counter::default()).await;

        let status = UserStatus::new("Counting ...", Presence::Busy);

        cx.edit(status).await.ok();
    }

    async fn message(&self, cx: Context, msg: Message) {
        let content = msg.content.to_string();

        if content == "!count" {
            cx.state.update(Counter::increment).await;

            let Counter(current_count) = cx.state.get().await.unwrap();
            let content = format!("#### Counted!\nCurrent count is **{current_count}**");

            msg.reply(&cx, content, false).await.ok();
        }
    }
}

#[tokio::main]
async fn main() -> Result {
    let token = env::var("TOKEN").unwrap();
    let mut client = Client::new(Handler, token).await?;

    client.listen().await
}
