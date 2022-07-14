use crate::{
    error::Result,
    http::{
        builders::{CreateChannel, CreateServer, EditServer},
        HttpClient, DELTA_API,
    },
    models::{Channel, Server},
};

impl HttpClient {
    /// Fetch a server.
    pub async fn server(&self, id: &str) -> Result<Server> {
        self.request(self.inner.get(format!("{DELTA_API}/servers/{id}")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Create a server.
    pub async fn create_server(&self, data: &CreateServer) -> Result<Server> {
        self.request(
            self.inner
                .post(format!("{DELTA_API}/servers/create"))
                .json(data),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
    }

    /// Edit a server.
    pub async fn edit_server(&self, id: &str, data: &EditServer) -> Result<Server> {
        self.request(
            self.inner
                .patch(format!("{DELTA_API}/servers/{id}"))
                .json(data),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
    }

    /// Mark all channels in a server as read.
    pub async fn read_server(&self, id: &str) -> Result {
        self.request(self.inner.put(format!("{DELTA_API}/servers/{id}/ack")))
            .await
            .map(|_| ())
    }

    /// Leaves a server.
    pub async fn leave_server(&self, id: &str) -> Result {
        self.request(self.inner.delete(format!("{DELTA_API}/servers/{id}")))
            .await
            .map(|_| ())
    }

    /// Create a server channel.
    pub async fn create_channel(&self, id: &str, data: &CreateChannel) -> Result<Channel> {
        self.request(
            self.inner
                .post(format!("{DELTA_API}/servers/{id}/channels"))
                .json(data),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
    }
}
