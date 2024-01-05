use tokio_postgres::{Client, NoTls, Error};

pub struct DBManager {
    client: Client,
}

impl DBManager {
    pub async fn new(uri: String) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(&uri, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {e}");
            }
        });

        Ok(DBManager {
            client,
        })
    }

    pub async fn create_user(&self, username: &String, password: &String) -> Result<(), Error> {

        let stmt = format!("
            create user {}
            with nocreatedb nocreaterole nosuperuser
            password '{}'
        ", username, password);

        self.client
            .execute(&stmt, &[]).await?;

        Ok(())
    }

    pub async fn create_db(&self, name: &String, owner: &String) -> Result<(), Error> {
        self.client.execute("
            create database $1
            with owner $2
        ", &[&name, &owner]).await?;

        Ok(())
    }
}
