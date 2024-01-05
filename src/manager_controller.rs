use futures::TryStreamExt;
use kube::Api;
use kube::api::ListParams;
use kube::runtime::watcher;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::crds::{Manager, ManagerSpec};
use crate::database::DBManager;

pub type ManagersMap = Arc<Mutex<HashMap<String, DBManager>>>;

pub async fn read_managers(managers_api: Api<Manager>) -> ManagersMap {
    let mut map = HashMap::<String, DBManager>::new();

    for m in managers_api.list(&ListParams::default()).await.unwrap() {
        let name = m.metadata.name.clone().unwrap();
        let uri = m.spec.uri;
        match DBManager::new(uri).await {
            Ok(manager) => {
                map.insert(name, manager);
            },
            Err(e) => {
                eprintln!("{e}")
            },
        };
    }

    Arc::new(Mutex::new(map))
}

async fn on_manager_edited(manager: Manager, managers_map: ManagersMap) -> bool {

    let name = manager.metadata.name.unwrap();
    let mut map = managers_map.lock().unwrap();
    if !map.contains_key(&name) {
        match DBManager::new(manager.spec.uri).await {
            Ok(manager) => {
                map.insert(name, manager);
                return true;
            },
            Err(e) => {
                eprintln!("{e}");
                return false;
            },
        };
    }
    true
}

async fn on_manager_removed(manager: Manager, managers_map: ManagersMap) -> bool {
    let name = manager.metadata.name.unwrap();

    match managers_map.lock().unwrap().remove(&name) {
        Some(_) => true,
        _ => false
    }
}

pub async fn watch_managers(managers_api: Api<Manager>, managers_map: ManagersMap) -> Result<bool, watcher::Error> {

    watcher(managers_api, watcher::Config::default())
        .try_all(|e| async {
            match e {
                watcher::Event::Applied(m)
                    => on_manager_edited(m, managers_map.clone()).await,
                watcher::Event::Deleted(m)
                    => on_manager_removed(m, managers_map.clone()).await,
                _ => true
            }
        }).await
}
