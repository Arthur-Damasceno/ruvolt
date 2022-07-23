use crate::{
    error::Result,
    http::{
        builders::{CreateMessage, EditMessage, FetchMessages, SearchMessages},
        HttpClient,
    },
    models::{message::Message, user::User, Member},
};

impl HttpClient {
    /// Fetch a message.
    pub async fn message(&self, channel_id: &str, id: &str) -> Result<Message> {
        self.request(self.get(format!("/channels/{channel_id}/messages/{id}")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Fetch multiple messages.
    pub async fn messages(
        &self,
        channel_id: &str,
        query: &FetchMessages,
    ) -> Result<MessagesAndUsers> {
        self.request(
            self.get(format!("/channels/{channel_id}/messages"))
                .query(query),
        )
        .await?
        .json::<MessagesResponse>()
        .await
        .map_err(From::from)
        .map(Into::into)
    }

    /// Search for messages.
    pub async fn search_messages(
        &self,
        channel_id: &str,
        query: &SearchMessages,
    ) -> Result<MessagesAndUsers> {
        self.request(
            self.post(format!("/channels/{channel_id}/search"))
                .json(query),
        )
        .await?
        .json::<MessagesResponse>()
        .await
        .map_err(From::from)
        .map(Into::into)
    }

    /// Send a message on a channel.
    pub async fn send_message(&self, channel_id: &str, data: &CreateMessage) -> Result<Message> {
        self.request(
            self.post(format!("/channels/{channel_id}/messages"))
                .json(data),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
    }

    /// Edit a message.
    pub async fn edit_message(
        &self,
        channel_id: &str,
        id: &str,
        data: &EditMessage,
    ) -> Result<Message> {
        self.request(
            self.patch(format!("/channels/{channel_id}/messages/{id}"))
                .json(data),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
    }

    /// Acknowledge a message as read.
    pub async fn acknowledge_message(&self, channel_id: &str, id: &str) -> Result {
        self.request(self.put(format!("/channels/{channel_id}/ack/{id}")))
            .await
            .map(|_| ())
    }

    /// Delete a message.
    pub async fn delete_message(&self, channel_id: &str, id: &str) -> Result {
        self.request(self.delete(format!("/channels/{channel_id}/messages/{id}")))
            .await
            .map(|_| ())
    }

    /// Delete multiple messages.
    pub async fn delete_messages(&self, channel_id: &str, ids: &[String]) -> Result {
        #[derive(Serialize)]
        struct Data<'a> {
            ids: &'a [String],
        }

        self.request(
            self.delete(format!("/channels/{channel_id}/messages/bulk"))
                .json(&Data { ids }),
        )
        .await
        .map(|_| ())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MessagesResponse {
    JustMessages(Vec<Message>),
    MessagesAndUsers {
        messages: Vec<Message>,
        users: Vec<User>,
        members: Option<Vec<Member>>,
    },
}

type MessagesAndUsers = (Vec<Message>, Option<Vec<User>>, Option<Vec<Member>>);

impl Into<MessagesAndUsers> for MessagesResponse {
    fn into(self) -> MessagesAndUsers {
        match self {
            Self::JustMessages(messages) => (messages, None, None),
            Self::MessagesAndUsers {
                messages,
                users,
                members,
            } => (messages, Some(users), members),
        }
    }
}
