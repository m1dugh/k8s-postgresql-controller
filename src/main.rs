mod crds;
mod manager_controller;
mod table_controller;

use crds::{Manager, Table};
use manager_controller::*;
use table_controller::watch_tables;

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
    let tables_api = Api::<Table>::all(client.clone());
    let managers_map = read_managers(managers_api.clone()).await;

    let managers_watcher = watch_managers(managers_api, managers_map.clone());

    let tables_watcher = watch_tables(tables_api, managers_map);

    join!(managers_watcher, tables_watcher);

    Ok(())
}
