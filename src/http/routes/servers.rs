use crate::{
    error::Result,
    http::{
        builders::{CreateChannel, CreateServer, EditServer},
        HttpClient,
    },
    models::{Channel, Server},
};

impl HttpClient {
    /// Fetch a server.
    pub async fn server(&self, id: &str) -> Result<Server> {
        self.request(self.get(format!("/servers/{id}")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Create a server.
    pub async fn create_server(&self, data: &CreateServer) -> Result<Server> {
        self.request(self.post("/servers/create").json(data))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Edit a server.
    pub async fn edit_server(&self, id: &str, data: &EditServer) -> Result<Server> {
        self.request(self.patch(format!("/servers/{id}")).json(data))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Mark all channels in a server as read.
    pub async fn read_server(&self, id: &str) -> Result {
        self.request(self.put(format!("/servers/{id}/ack")))
            .await
            .map(|_| ())
    }

    /// Leaves a server.
    pub async fn leave_server(&self, id: &str) -> Result {
        self.request(self.delete(format!("/servers/{id}")))
            .await
            .map(|_| ())
    }

    /// Create a server channel.
    pub async fn create_channel(&self, id: &str, data: &CreateChannel) -> Result<Channel> {
        self.request(self.post(format!("/servers/{id}/channels")).json(data))
            .await?
            .json()
            .await
            .map_err(From::from)
    }
}
