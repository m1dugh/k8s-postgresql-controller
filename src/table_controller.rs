use futures::TryStreamExt;
use kube::{ResourceExt, runtime::watcher, Api};

use crate::{crds::Table, manager_controller::ManagersMap};

const ANNOTATION_KEY: &str = "psql.midugh.fr/manager";

async fn on_table_applied(table: Table, managers_map: ManagersMap) -> bool {

    let namespace = table.namespace().clone().unwrap();

    let annotations = match table.metadata.annotations {
        Some(v) => v,
        _ => return false,
    };

    let manager_name = match annotations.get(ANNOTATION_KEY) {
        Some(v) => v,
        _ => return false,
    };

    let manager = {
        match managers_map.lock().unwrap().get(manager_name) {
            Some(v) => v.clone(),
            _ => return false,
        }
    };

    println!("found manager {manager:?} for table {:?}", table.spec);

    true
}

async fn on_table_removed(_table: Table, _managers_map: ManagersMap) -> bool {
    true
}

pub async fn watch_tables(tables_api: Api<Table>, managers_map: ManagersMap) -> Result<bool, watcher::Error> {
    watcher(tables_api, watcher::Config::default())
        .try_all(|e| async {
            match e {
                watcher::Event::Applied(t)
                    => on_table_applied(t, managers_map.clone()).await,
                watcher::Event::Deleted(t)
                    => on_table_removed(t, managers_map.clone()).await,
                _ => true,
            }
        }).await
}
