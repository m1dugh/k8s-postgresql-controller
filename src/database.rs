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

        // TODO: sanitize input
        let stmt = format!("
            create user {}
            with nocreatedb nocreaterole nosuperuser
            password '{}'
        ", username, password);

        self.client
            .execute(&stmt, &[]).await?;

        Ok(())
    }

    pub async fn delete_user(&self, username: &String) -> Result<(), Error> {
        let stmt = format!("
            drop user {}
        ", username);

        self.client
            .execute(&stmt, &[]).await?;
        Ok(())
    }

    pub async fn create_db(&self, name: &String, owner: &String) -> Result<(), Error> {

        // TODO: sanitize input

        let stmt = format!("
            create database {}
            with owner {}
        ", name, owner);

        self.client
            .execute(&stmt, &[])
            .await?;

        Ok(())
    }

    pub async fn delete_db(&self, name: &String) -> Result<(), Error> {
        // TODO: sanitize input
        let stmt = format!("drop db {}", name);

        self.client
            .execute(&stmt, &[])
            .await?;

        Ok(())
    }
}
