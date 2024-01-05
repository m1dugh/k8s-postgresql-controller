mod crds;
mod manager_controller;
mod database_controller;
mod types;
mod database;

use crate::crds::{Manager, Database};
use crate::manager_controller::*;
use crate::database_controller::watch_databases;

use futures::join;

use kube::{
    api::Api,
    Client,
    runtime::watcher,
};

#[tokio::main]
async fn main() -> Result<(), watcher::Error>
{

    let client = Client::try_default().await.unwrap();
    let managers_api = Api::<Manager>::all(client.clone());
    let db_api = Api::<Database>::all(client.clone());
    let managers_map = read_managers(managers_api.clone()).await;

    let managers_watcher = watch_managers(managers_api, managers_map.clone());

    let db_watcher = watch_databases(db_api, managers_map);

    join!(managers_watcher, db_watcher);

    Ok(())
}
