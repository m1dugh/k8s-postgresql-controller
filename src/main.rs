mod crds;
mod manager_controller;
mod database_controller;
mod types;
mod database;
mod server;
mod error;

use crate::crds::{Manager, Database};
use crate::manager_controller::*;
use crate::database_controller::watch_databases;
use crate::error::{Error, Result};

use futures::join;

use kube::{
    api::Api,
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Error>
{

    let client = Client::try_default().await.unwrap();
    let managers_api = Api::<Manager>::all(client.clone());
    let db_api = Api::<Database>::all(client.clone());
    let managers_map = read_managers(managers_api.clone()).await;

    let managers_watcher = watch_managers(managers_api, managers_map.clone());

    let db_watcher = watch_databases(db_api, managers_map);

    let http_server = server::start_server("0.0.0.0", 8080);

    let (r_manager, r_db, r_rocket) = join!(managers_watcher, db_watcher, http_server);

    if let Err(e) = r_manager {
        return Err(Error::Default(format!("manager error: {e}")));
    }

    if let Err(e) = r_db {
        return Err(Error::Default(format!("db error: {e}")));
    }

    if let Err(e) = r_rocket {
        return Err(Error::Default(format!("server error: {e}")));
    }

    Ok(())
}
